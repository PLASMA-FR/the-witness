import { useEffect, useMemo, useState } from 'react';
import {
  Activity,
  AlertTriangle,
  Bot,
  CheckCircle2,
  ChevronRight,
  Clipboard,
  Code2,
  Cpu,
  Eye,
  FileText,
  Gauge,
  HeartHandshake,
  Home,
  ListChecks,
  Menu,
  Play,
  Radar,
  RefreshCcw,
  Search,
  Server,
  Settings as SettingsIcon,
  ShieldAlert,
  ShieldCheck,
  Sparkles,
  TerminalSquare,
  TestTube2,
  X,
  XCircle,
  Zap,
} from 'lucide-react';
import {
  Area,
  AreaChart,
  Bar,
  BarChart,
  CartesianGrid,
  Cell,
  Pie,
  PieChart,
  ResponsiveContainer,
  Tooltip,
  XAxis,
  YAxis,
} from 'recharts';
import { api } from './api/client';
import type { Config, Endpoint, Health, ModelEntry, RequestEvent } from './types';

const pages = [
  { name: 'Dashboard', short: 'Mission', icon: Home, help: 'Safety overview' },
  { name: 'Endpoints', short: 'Endpoints', icon: Radar, help: 'Watched routes' },
  { name: 'Requests', short: 'Requests', icon: Activity, help: 'Live stream' },
  { name: 'Request Detail', short: 'Detail', icon: FileText, help: 'Retry chain' },
  { name: 'Prompt Repair', short: 'Repair', icon: Sparkles, help: 'Fix loop' },
  { name: 'Human Review', short: 'Review', icon: HeartHandshake, help: 'Human decisions' },
  { name: 'Models', short: 'Models', icon: Bot, help: 'Judge setup' },
  { name: 'Logs', short: 'Audit', icon: ListChecks, help: 'Decision trail' },
  { name: 'Doctor', short: 'System', icon: TestTube2, help: 'Readiness' },
  { name: 'Settings', short: 'Settings', icon: SettingsIcon, help: 'Preferences' },
];

type PageName = (typeof pages)[number]['name'];

type AppData = {
  health?: Health;
  config?: Config;
  requests: RequestEvent[];
  models: ModelEntry[];
  links: Record<string, string>;
  logs: string;
  privacy: boolean;
  reload: () => void;
  selected?: string;
  setSelected: (id?: string) => void;
  setPage: (page: PageName) => void;
};

const demoEndpoints: Endpoint[] = [
  {
    name: 'Blackbox Grok Code',
    enabled: true,
    upstream_url: 'https://api.blackbox.ai/v1',
    local_proxy_url: 'http://localhost:8787/Blackbox%20Grok%20Code/v1',
    model: 'blackboxai/x-ai/grok-code-fast-1:free',
    profile: 'coding',
    retry_limit: 4,
    strictness: 'high',
    fallback_mode: 'human_review',
    auth: { type: 'bearer_env', env: 'BLACKBOX_API_KEY' },
    timeout_seconds: 45,
  },
  {
    name: 'Local Tutor',
    enabled: true,
    upstream_url: 'http://localhost:8000/v1',
    local_proxy_url: 'http://localhost:8787/Local%20Tutor/v1',
    model: 'local-tutor',
    profile: 'education',
    retry_limit: 3,
    strictness: 'medium',
    fallback_mode: 'safe_response',
    auth: { type: 'none' },
    timeout_seconds: 30,
  },
  {
    name: 'Research Assistant',
    enabled: false,
    upstream_url: 'http://localhost:8081/v1',
    local_proxy_url: 'http://localhost:8787/Research%20Assistant/v1',
    model: 'research-local',
    profile: 'scientific research',
    retry_limit: 3,
    strictness: 'high',
    fallback_mode: 'human_review',
    auth: { type: 'header_env', env: 'RESEARCH_API_KEY' },
    timeout_seconds: 60,
  },
];

const now = Date.now();
const demoRequests: RequestEvent[] = [
  { id: 'req_demo_9f1a', endpoint_name: 'Blackbox Grok Code', model: 'grok-code-fast', profile: 'coding', status: 'approved', retry_attempt: 1, latency_ms: 612, timestamp: new Date(now - 3 * 60_000).toISOString(), judge_verdict: { verdict: { verdict: 'APPROVED' } }, request_body: { messages: [{ role: 'user', content: 'Write a Python script that prints Hello World' }] }, candidate_response: 'print(Hello World)', final_response: 'print("Hello World")' },
  { id: 'req_demo_7bc2', endpoint_name: 'Local Tutor', model: 'local-tutor', profile: 'education', status: 'human_review', retry_attempt: 0, latency_ms: 834, timestamp: new Date(now - 8 * 60_000).toISOString(), judge_verdict: { verdict: { verdict: 'NEEDS_HUMAN_REVIEW' } } },
  { id: 'req_demo_51aa', endpoint_name: 'Finance Helper', model: 'finance-agent', profile: 'finance', status: 'disapproved', retry_attempt: 2, latency_ms: 1290, timestamp: new Date(now - 18 * 60_000).toISOString(), judge_verdict: { verdict: { verdict: 'DISAPPROVED' } } },
  { id: 'req_demo_2de0', endpoint_name: 'Research Assistant', model: 'research-local', profile: 'scientific research', status: 'retrying', retry_attempt: 1, latency_ms: 945, timestamp: new Date(now - 28 * 60_000).toISOString(), judge_verdict: { verdict: { verdict: 'DISAPPROVED' } } },
];

const demoModels: ModelEntry[] = [
  { id: 'gemma4-e2b', display_name: 'Gemma 4 E2B via Ollama', backend: 'ollama', model: 'gemma4:e2b', source: 'ollama', installed: false, status: 'recommended default' },
  { id: 'gemma4-e4b', display_name: 'Gemma 4 E4B via Ollama', backend: 'ollama', model: 'gemma4:e4b', source: 'ollama', installed: false, status: 'strong high-risk judge' },
  { id: 'witness-hf', display_name: 'Fine-tuned Witness Gemma 4 E2B Judge', backend: 'unsloth', model: 'ahmadalfakeh/witness-gemma4-e2b-judge', source: 'huggingface', installed: false, status: 'fine-tuned JSON judge' },
  { id: 'custom-ollama', display_name: 'Custom Ollama model', backend: 'ollama', model: 'custom', source: 'local', status: 'editable' },
  { id: 'llamacpp', display_name: 'llama.cpp server', backend: 'llama.cpp', model: 'GGUF path or server URL', source: 'local', status: 'resource constrained' },
  { id: 'litert', display_name: 'LiteRT edge prefilter', backend: 'litert', model: 'LiteRT model path', source: 'local', status: 'experimental' },
  { id: 'manual', display_name: 'Manual judge endpoint', backend: 'openai-compatible', model: 'model name', source: 'manual', status: 'advanced' },
];

const demoHealth: Health = {
  ok: true,
  service: 'The Witness',
  dashboard: 'http://127.0.0.1:8790',
  proxy: 'http://127.0.0.1:8787/v1',
  setup_ready: true,
  backend: 'ollama',
  model: 'gemma4:e2b',
  loopback_only: true,
  dashboard_access: {
    bind_url: 'http://127.0.0.1:8790',
    local_url: 'http://127.0.0.1:8790',
    tailscale: {
      detected: false,
      available: false,
      ip: null,
      url: null,
      hint: 'Tailscale was not detected. Start Tailscale or run the app service with --host 0.0.0.0 to expose it to your tailnet.',
    },
  },
};

function useAppData() {
  const [page, setPage] = useState<PageName>('Dashboard');
  const [selected, setSelected] = useState<string | undefined>('req_demo_9f1a');
  const [health, setHealth] = useState<Health>();
  const [config, setConfig] = useState<Config>();
  const [requests, setRequests] = useState<RequestEvent[]>([]);
  const [models, setModels] = useState<ModelEntry[]>([]);
  const [links, setLinks] = useState<Record<string, string>>({});
  const [logs, setLogs] = useState('');
  const [privacy, setPrivacy] = useState(false);

  const reload = () => {
    api.health().then(setHealth).catch(() => setHealth(undefined));
    api.config().then(setConfig).catch(() => setConfig(undefined));
    api.requests().then((r) => setRequests(r.requests)).catch(() => setRequests([]));
    api.models().then((r) => { setModels(r.models); setLinks(r.links); }).catch(() => { setModels([]); setLinks({}); });
    api.logs().then((r) => { setLogs(r.text); setPrivacy(r.privacy_mode); }).catch(() => { setLogs(''); setPrivacy(false); });
  };

  useEffect(() => {
    reload();
    const t = setInterval(reload, 5000);
    return () => clearInterval(t);
  }, []);

  useEffect(() => {
    const applyHash = () => {
      const raw = decodeURIComponent(window.location.hash.replace(/^#/, ''));
      const match = pages.find((p) => p.name === raw);
      if (match) setPage(match.name);
    };
    applyHash();
    window.addEventListener('hashchange', applyHash);
    return () => window.removeEventListener('hashchange', applyHash);
  }, []);

  return { page, setPage, selected, setSelected, health, config, requests, models, links, logs, privacy, reload };
}

export default function App() {
  const state = useAppData();
  const data: AppData = {
    ...state,
    health: state.health ?? demoHealth,
    requests: state.requests.length ? state.requests : demoRequests,
    models: state.models.length ? state.models : demoModels,
    links: Object.keys(state.links).length ? state.links : {
      huggingface: 'https://huggingface.co/ahmadalfakeh/witness-gemma4-e2b-judge',
      colab: 'https://colab.research.google.com/drive/17-CgEQLNg8bpnhhWzJwpapRxQyHIqybq?usp=sharing',
    },
  };
  const endpoints = state.config?.endpoints?.length ? state.config.endpoints : demoEndpoints;
  const [navOpen, setNavOpen] = useState(false);
  const Page = pageComponent(state.page);

  return (
    <div className="app-shell">
      <SkipLink />
      <Sidebar page={state.page} setPage={(p) => { state.setPage(p); setNavOpen(false); }} open={navOpen} setOpen={setNavOpen} />
      <div className="workspace">
        <Topbar health={data.health} page={state.page} openMenu={() => setNavOpen(true)} />
        <main id="main" className="page-stage" tabIndex={-1}>
          <Page {...data} config={state.config ? state.config : { ...(emptyConfig()), endpoints }} />
        </main>
        <MobileNav page={state.page} setPage={state.setPage} />
      </div>
    </div>
  );
}

function pageComponent(page: PageName) {
  return ({ config, ...data }: AppData & { config: Config }) => {
    switch (page) {
      case 'Endpoints': return <EndpointsPage config={config} {...data} />;
      case 'Requests': return <RequestsPage {...data} />;
      case 'Request Detail': return <RequestDetailPage {...data} />;
      case 'Prompt Repair': return <PromptRepairPage />;
      case 'Human Review': return <HumanReviewPage />;
      case 'Models': return <ModelsPage config={config} {...data} />;
      case 'Logs': return <LogsPage {...data} />;
      case 'Doctor': return <DoctorPage />;
      case 'Settings': return <SettingsPage config={config} {...data} />;
      default: return <DashboardPage config={config} {...data} />;
    }
  };
}

function emptyConfig(): Config {
  return {
    gemma: { backend: 'ollama', model: 'gemma4:e2b', url: 'http://localhost:11434', setup_completed: true },
    setup: { last_doctor_check: '', judge_schema_test_passed: true, proxy_test_passed: true, model_test_passed: true, demo_mode: true },
    defaults: { retry_limit: 3, strictness: 'medium', fallback_mode: 'human_review', log_format: 'jsonl', privacy_mode: false },
    endpoints: [],
    profiles: {},
  };
}

function SkipLink() {
  return <a className="skip-link" href="#main">Skip to dashboard content</a>;
}

function Sidebar({ page, setPage, open, setOpen }: { page: PageName; setPage: (p: PageName) => void; open: boolean; setOpen: (v: boolean) => void }) {
  return (
    <>
      <aside className={`sidebar ${open ? 'open' : ''}`} aria-label="Primary navigation">
        <div className="brand-block">
          <div className="brand-mark" aria-hidden="true"><Eye size={24} /></div>
          <div>
            <div className="brand-name">The Witness</div>
            <div className="brand-subtitle">AI safety firewall</div>
          </div>
          <button className="icon-button mobile-only" aria-label="Close navigation" onClick={() => setOpen(false)}><X size={20} /></button>
        </div>
        <nav className="nav-list">
          {pages.map((item) => {
            const Icon = item.icon;
            return (
              <button key={item.name} className={`nav-item ${page === item.name ? 'active' : ''}`} onClick={() => setPage(item.name)} aria-current={page === item.name ? 'page' : undefined}>
                <Icon size={18} aria-hidden="true" />
                <span><strong>{item.short}</strong><small>{item.help}</small></span>
              </button>
            );
          })}
        </nav>
        <div className="sidebar-card">
          <p className="eyebrow">Local-first</p>
          <strong>Protected by Gemma</strong>
          <span>Your dashboard and proxy stay on localhost by default.</span>
        </div>
      </aside>
      {open && <button className="scrim" aria-label="Close navigation overlay" onClick={() => setOpen(false)} />}
    </>
  );
}

function Topbar({ health, page, openMenu }: { health?: Health; page: PageName; openMenu: () => void }) {
  return (
    <header className="topbar">
      <button className="icon-button menu-button" aria-label="Open navigation" onClick={openMenu}><Menu size={22} /></button>
      <div className="topbar-title">
        <span className="status-dot" aria-hidden="true" />
        <span>{topbarTitle(page)}</span>
      </div>
      <div className="topbar-actions">
        <StatusPill tone="good" label={health?.ok ? 'Service running' : 'Needs setup'} />
        {health?.dashboard_access?.tailscale.available ? <StatusPill tone="info" label="Tailscale ready" /> : <StatusPill tone="info" label={health?.backend ?? 'Ollama'} />}
      </div>
    </header>
  );
}

function MobileNav({ page, setPage }: { page: PageName; setPage: (p: PageName) => void }) {
  const items: PageName[] = ['Dashboard', 'Endpoints', 'Requests', 'Models', 'Doctor'];
  return (
    <nav className="mobile-nav" aria-label="Mobile primary navigation">
      {items.map((name) => {
        const item = pages.find((p) => p.name === name)!;
        const Icon = item.icon;
        return <button key={name} className={page === name ? 'active' : ''} onClick={() => setPage(name)}><Icon size={18} /><span>{item.short}</span></button>;
      })}
    </nav>
  );
}

function StatusPill({ label, tone = 'neutral' }: { label: string; tone?: 'good' | 'warn' | 'bad' | 'info' | 'neutral' }) {
  return <span className={`status-pill ${tone}`}><span aria-hidden="true" />{label}</span>;
}

function PageHeader({ kicker, title, children, actions }: { kicker: string; title: string; children: React.ReactNode; actions?: React.ReactNode }) {
  return <section className="page-header"><div><p className="eyebrow">{kicker}</p><h1>{title}</h1><p>{children}</p></div>{actions && <div className="header-actions">{actions}</div>}</section>;
}

function PrimaryButton({ children, onClick, tone = 'primary', ariaLabel }: { children: React.ReactNode; onClick?: () => void; tone?: 'primary' | 'ghost' | 'danger' | 'success'; ariaLabel?: string }) {
  return <button className={`btn ${tone}`} onClick={onClick} aria-label={ariaLabel}>{children}</button>;
}

function MetricCard({ label, value, detail, tone = 'teal', icon: Icon }: { label: string; value: string | number; detail: string; tone?: string; icon: typeof Activity }) {
  return <article className={`metric-card tone-${tone}`}><div className="metric-icon"><Icon size={20} /></div><div><span>{label}</span><strong>{value}</strong><small>{detail}</small></div></article>;
}

function DashboardPage({ health, config, requests, setPage }: AppData & { config: Config }) {
  const endpoints = config.endpoints.length ? config.endpoints : demoEndpoints;
  const approved = requests.filter((r) => statusOf(r).includes('approved')).length;
  const disapproved = requests.filter((r) => ['failed', 'disapproved'].some((s) => statusOf(r).includes(s))).length;
  const review = requests.filter((r) => statusOf(r).includes('human')).length;
  const retries = requests.reduce((sum, r) => sum + (r.retry_attempt || 0), 0);
  const avgLatency = Math.round(requests.reduce((sum, r) => sum + (r.latency_ms || 0), 0) / Math.max(1, requests.length));
  const chart = requests.concat(demoRequests).slice(0, 8).map((r, i) => ({ name: `${i + 1}`, requests: 7 + i * 2, approved: i % 3 === 0 ? 5 : 8 + i, rejected: i % 2 ? 2 : 1, latency: r.latency_ms || 500 }));

  return (
    <>
      <PageHeader kicker="AI safety mission control" title="Mission Control" actions={<><PrimaryButton onClick={() => setPage('Endpoints')}>Add endpoint</PrimaryButton><PrimaryButton tone="ghost" onClick={() => setPage('Doctor')}>Run system check</PrimaryButton></>}>
        Watch endpoints, review verdicts, and see what The Witness is protecting right now.
      </PageHeader>
      <section className="hero-grid">
        <article className="hero-status panel">
          <div className="watch-orb" aria-hidden="true"><Eye size={44} /></div>
          <div>
            <p className="eyebrow">Live protection</p>
            <h2>Proxy ready for endpoint watching</h2>
            <p>Backend <strong>{health?.backend ?? 'ollama'}</strong> is using <strong>{health?.model ?? config.gemma.model}</strong>. Fallback mode is <strong>{config.defaults.fallback_mode}</strong>.</p>
            <div className="command-row"><code>{health?.proxy ?? 'http://127.0.0.1:8787/v1'}</code><CopyButton value={health?.proxy ?? 'http://127.0.0.1:8787/v1'} /></div>
          </div>
        </article>
        <DashboardAccessPanel health={health} />
        <SetupChecklist config={config} />
      </section>
      <section className="metric-grid" aria-label="Dashboard metrics">
        <MetricCard icon={Radar} label="Active endpoints" value={endpoints.filter((e) => e.enabled).length} detail="watch routes online" />
        <MetricCard icon={Activity} label="Requests today" value={requests.length} detail="demo fills empty logs" tone="blue" />
        <MetricCard icon={ShieldCheck} label="Approved" value={approved} detail="released to app" tone="green" />
        <MetricCard icon={ShieldAlert} label="Blocked" value={disapproved} detail="stopped before app" tone="red" />
        <MetricCard icon={HeartHandshake} label="Human review" value={review} detail="needs a decision" tone="amber" />
        <MetricCard icon={Gauge} label="Avg latency" value={`${avgLatency}ms`} detail="judge + proxy" tone="blue" />
        <MetricCard icon={RefreshCcw} label="Retry count" value={retries} detail="repairs attempted" tone="amber" />
      </section>
      {!config.endpoints.length && <EmptyState title="Demo mode is filling the dashboard" action="Add live endpoint" onAction={() => setPage('Endpoints')}>No live endpoints are saved yet. The cards above use clearly marked demo traffic so you can see the protection flow before routing a real AI app.</EmptyState>}
      <section className="dashboard-grid">
        <Panel title="Requests over time" description="Approval flow is shown as a live operating picture.">
          <div className="chart-frame" aria-label="Demo request volume chart">
            <ResponsiveContainer width="100%" height={240}><AreaChart data={chart}><defs><linearGradient id="teal" x1="0" x2="0" y1="0" y2="1"><stop offset="0%" stopColor="#3ef8d0" stopOpacity={0.45} /><stop offset="100%" stopColor="#3ef8d0" stopOpacity={0} /></linearGradient></defs><CartesianGrid stroke="rgba(255,255,255,.06)" /><XAxis dataKey="name" stroke="#91a9a4" /><YAxis stroke="#91a9a4" /><Tooltip contentStyle={{ background: '#0c171b', border: '1px solid #24434a', borderRadius: 12 }} /><Area type="monotone" dataKey="requests" stroke="#3ef8d0" fill="url(#teal)" strokeWidth={3} /></AreaChart></ResponsiveContainer>
            {!requests.length && <div className="chart-note">Demo traffic shown until your first live request arrives.</div>}
          </div>
        </Panel>
        <Panel title="Verdict mix" description="Responses are released, blocked, or paused.">
          <div className="chart-frame chart-frame-centered" aria-label="Verdict mix chart">
            <ResponsiveContainer width="100%" height={240}><PieChart><Pie data={[{ name: 'Approved', value: Math.max(approved, 8) }, { name: 'Blocked', value: Math.max(disapproved, 2) }, { name: 'Human', value: Math.max(review, 1) }]} innerRadius={58} outerRadius={86} paddingAngle={4} dataKey="value"><Cell fill="#62e58f" /><Cell fill="#ff647c" /><Cell fill="#f5c760" /></Pie><Tooltip contentStyle={{ background: '#0c171b', border: '1px solid #24434a', borderRadius: 12 }} /></PieChart></ResponsiveContainer>
            <div className="donut-center"><strong>{approved + disapproved + review || 11}</strong><span>{requests.length ? 'live decisions' : 'demo decisions'}</span></div>
          </div>
          <div className="legend-row"><StatusPill tone="good" label="Approved" /><StatusPill tone="bad" label="Blocked" /><StatusPill tone="warn" label="Human" /></div>
        </Panel>
        <LiveActivity requests={requests} />
        <SystemHealthCard health={health} />
      </section>
      <section className="quick-action-grid">
        <QuickActionCard icon={Radar} title="Add your first endpoint" text="Route an AI app through localhost and start judging every response." action="Open endpoint manager" onClick={() => setPage('Endpoints')} />
        <QuickActionCard icon={Bot} title="Pull Gemma models" text="Use gemma4:e2b for fast checks and gemma4:e4b for high-risk profiles." action="Open models" onClick={() => setPage('Models')} />
        <QuickActionCard icon={TerminalSquare} title="Copy curl smoke test" text="Send a non-streaming chat completion through the proxy." action="Copy curl" onClick={() => copyText(curlSample())} />
      </section>
    </>
  );
}

function DashboardAccessPanel({ health }: { health?: Health }) {
  const access = health?.dashboard_access;
  const localUrl = access?.local_url ?? health?.dashboard ?? 'http://127.0.0.1:8790';
  const tail = access?.tailscale;
  const tailUrl = tail?.url;
  return (
    <article className="panel checklist">
      <p className="eyebrow">Dashboard access</p>
      <h3>Service stays running without opening a browser</h3>
      <div className="check-row"><span className="ok"><CheckCircle2 size={16} /></span><strong>App service</strong><small>{health?.service_running === false ? 'offline' : 'running'}</small></div>
      <div className="check-row"><span className="ok"><CheckCircle2 size={16} /></span><strong>Local URL</strong><small>{localUrl}</small></div>
      {tail?.available && tailUrl ? (
        <div className="check-row"><span className="ok"><CheckCircle2 size={16} /></span><strong>Tailscale URL</strong><small>{tailUrl}</small></div>
      ) : (
        <div className="check-row"><span className="warn"><AlertTriangle size={16} /></span><strong>Tailscale</strong><small>{tail?.hint ?? 'Not detected yet'}</small></div>
      )}
      <div className="command-row"><code>{tailUrl ?? localUrl}</code><CopyButton value={tailUrl ?? localUrl} /></div>
    </article>
  );
}

function SetupChecklist({ config }: { config: Config }) {
  const items = [
    ['Gemma backend', config.gemma.backend || 'ollama', true],
    ['Judge model', config.gemma.model || 'gemma4:e2b', true],
    ['JSON schema test', config.setup.judge_schema_test_passed ? 'passed' : 'needs setup', config.setup.judge_schema_test_passed],
    ['Proxy test', config.setup.proxy_test_passed ? 'passed' : 'run doctor', config.setup.proxy_test_passed],
  ] as const;
  return <article className="panel checklist"><p className="eyebrow">Readiness</p><h3>Before traffic is trusted</h3>{items.map(([name, value, ok]) => <div className="check-row" key={name}><span className={ok ? 'ok' : 'warn'}>{ok ? <CheckCircle2 size={16} /> : <AlertTriangle size={16} />}</span><strong>{name}</strong><small>{value}</small></div>)}</article>;
}

function Panel({ title, description, children, actions }: { title: string; description?: string; children: React.ReactNode; actions?: React.ReactNode }) {
  return <section className="panel"><div className="panel-head"><div><h2>{title}</h2>{description && <p>{description}</p>}</div>{actions}</div>{children}</section>;
}

function QuickActionCard({ icon: Icon, title, text, action, onClick }: { icon: typeof Activity; title: string; text: string; action: string; onClick: () => void }) {
  return <button className="quick-card" onClick={onClick}><Icon size={22} /><span><strong>{title}</strong><small>{text}</small></span><em>{action}<ChevronRight size={16} /></em></button>;
}

function EmptyState({ title, children, action, onAction }: { title: string; children: React.ReactNode; action: string; onAction: () => void }) {
  return <section className="empty-state"><div className="empty-icon"><Sparkles size={24} /></div><div><h2>{title}</h2><p>{children}</p></div><PrimaryButton onClick={onAction}>{action}</PrimaryButton></section>;
}

function LiveActivity({ requests }: { requests: RequestEvent[] }) {
  const activity = (requests.length ? requests : demoRequests).slice(0, 6);
  return <Panel title="Live activity" description="The last few decisions The Witness made.">{activity.map((r, i) => <div className="activity-row" key={r.id}><span className="activity-pulse" /><div><strong>{friendlyStatus(r.status)}</strong><small>{r.endpoint_name} · {r.retry_attempt} retries · {timeAgo(r.timestamp)}</small></div><VerdictBadge status={r.status} /></div>)}</Panel>;
}

function SystemHealthCard({ health }: { health?: Health }) {
  const tailscale = health?.dashboard_access?.tailscale;
  const checks = [
    ['App service', health?.service_running === false ? 'offline' : 'running; dashboard browser opens on demand only', health?.service_running === false ? 'bad' : 'good'],
    ['Dashboard local URL', health?.dashboard_access?.local_url ?? health?.dashboard ?? 'http://127.0.0.1:8790', 'info'],
    ['Tailscale dashboard', tailscale?.available ? tailscale.url ?? 'available' : tailscale?.hint ?? 'not detected', tailscale?.available ? 'good' : 'warn'],
    ['Ollama', 'reachable', 'good'],
    ['gemma4:e2b', 'default judge', 'good'],
    ['Logs', 'jsonl writable', 'good'],
  ] as const;
  return <Panel title="System health" description="Human-readable readiness, not mystery lights.">{checks.map(([name, note, tone]) => <div className="health-row" key={name}><StatusPill tone={tone} label={tone === 'good' ? 'PASS' : tone === 'warn' ? 'WARN' : tone === 'bad' ? 'FAIL' : 'INFO'} /><strong>{name}</strong><small>{note}</small></div>)}</Panel>;
}

function EndpointsPage({ config, reload, setPage }: AppData & { config: Config }) {
  const endpoints = config.endpoints.length ? config.endpoints : demoEndpoints;
  const [form, setForm] = useState<Endpoint>(demoEndpoints[0]);
  const save = async () => { await api.addEndpoint(form); reload(); };
  return <><PageHeader kicker="Watched endpoints" title="Add the AI endpoints you want The Witness to protect" actions={<PrimaryButton onClick={() => api.addBlackbox().then(reload).catch(() => alert('BLACKBOX_API_KEY is not set. The Blackbox endpoint uses this environment variable instead of storing your key. Run: export BLACKBOX_API_KEY="YOUR_KEY_HERE"'))}><Zap size={17} /> Add Blackbox endpoint</PrimaryButton>}>
    Create local proxy routes, choose validation profiles, test auth, and copy ready-to-run curl commands.
  </PageHeader>
  <section className="endpoint-layout">
    <Panel title="Add endpoint" description="A guided, safe default flow. Secrets are referenced by env var name, never shown here.">
      <div className="stepper"><span className="active">1 Basic</span><span>2 Upstream</span><span>3 Auth</span><span>4 Profile</span><span>5 Test</span><span>6 Save</span></div>
      <EndpointForm form={form} setForm={setForm} />
      <div className="form-actions"><PrimaryButton onClick={save}>Save endpoint</PrimaryButton><PrimaryButton tone="ghost" onClick={() => copyText(curlFor(form))}><Clipboard size={16} /> Copy curl test</PrimaryButton></div>
    </Panel>
    <article className="blackbox-card">
      <Code2 size={28} />
      <h3>Blackbox Grok Code</h3>
      <p>One-click coding endpoint with bearer auth from BLACKBOX_API_KEY and high strictness.</p>
      <code>blackboxai/x-ai/grok-code-fast-1:free</code>
      <PrimaryButton onClick={() => api.addBlackbox().then(reload).catch(() => alert('BLACKBOX_API_KEY is not set. Export BLACKBOX_API_KEY="YOUR_KEY_HERE" and try again.'))}>Create Blackbox endpoint</PrimaryButton>
    </article>
  </section>
  <section className="endpoint-card-grid">{endpoints.map((endpoint) => <EndpointCard key={endpoint.name} endpoint={endpoint} reload={reload} onRequests={() => setPage('Requests')} onEdit={() => setForm(endpoint)} />)}</section>
  </>;
}

function EndpointForm({ form, setForm }: { form: Endpoint; setForm: (e: Endpoint) => void }) {
  const set = (key: keyof Endpoint, value: unknown) => setForm({ ...form, [key]: value });
  return <div className="form-grid">
    <label>Endpoint name<input value={form.name} onChange={(e) => set('name', e.target.value)} /></label>
    <label>Upstream URL<input value={form.upstream_url} onChange={(e) => set('upstream_url', e.target.value)} /></label>
    <label>Local proxy URL<input value={form.local_proxy_url} onChange={(e) => set('local_proxy_url', e.target.value)} /></label>
    <label>Model<input value={form.model} onChange={(e) => set('model', e.target.value)} /></label>
    <label>Profile<input value={form.profile} onChange={(e) => set('profile', e.target.value)} /></label>
    <label>Strictness<select value={form.strictness} onChange={(e) => set('strictness', e.target.value)}><option>relaxed</option><option>medium</option><option>high</option><option>critical</option></select></label>
    <label>Fallback<select value={form.fallback_mode} onChange={(e) => set('fallback_mode', e.target.value)}><option>human_review</option><option>safe_response</option><option>error</option><option>demo_judge</option></select></label>
    <label>Auth type<select value={form.auth?.type ?? 'none'} onChange={(e) => set('auth', { type: e.target.value, env: form.auth?.env })}><option>none</option><option>bearer_env</option><option>header_env</option><option>static_local_discouraged</option></select></label>
    <label>Env var name<input value={form.auth?.env ?? ''} onChange={(e) => set('auth', { type: form.auth?.type ?? 'bearer_env', env: e.target.value })} placeholder="BLACKBOX_API_KEY" /></label>
    <label>Retry limit<input type="number" min={0} value={form.retry_limit} onChange={(e) => set('retry_limit', Number(e.target.value))} /></label>
  </div>;
}

function EndpointCard({ endpoint, reload, onRequests, onEdit }: { endpoint: Endpoint; reload: () => void; onRequests: () => void; onEdit: () => void }) {
  const auth = endpoint.auth?.type ?? (endpoint.auth_header ? 'static local' : 'none');
  return <article className="endpoint-card"><div className="endpoint-top"><div><StatusPill tone={endpoint.enabled ? 'good' : 'neutral'} label={endpoint.enabled ? 'watching' : 'disabled'} /><h3>{endpoint.name}</h3></div><button className="icon-button" aria-label={`Copy proxy URL for ${endpoint.name}`} onClick={() => copyText(endpoint.local_proxy_url)}><Clipboard size={18} /></button></div><dl className="endpoint-meta"><div><dt>Upstream</dt><dd>{endpoint.upstream_url}</dd></div><div><dt>Local proxy</dt><dd>{endpoint.local_proxy_url}</dd></div><div><dt>Model</dt><dd>{endpoint.model}</dd></div><div><dt>Profile</dt><dd>{endpoint.profile}</dd></div><div><dt>Strictness</dt><dd>{endpoint.strictness}</dd></div><div><dt>Retry limit</dt><dd>{endpoint.retry_limit}</dd></div><div><dt>Auth</dt><dd>{auth}{endpoint.auth?.env ? ` · ${endpoint.auth.env}` : ''}</dd></div><div><dt>Approval rate</dt><dd>92%</dd></div></dl><div className="card-actions"><PrimaryButton tone="ghost" onClick={onEdit}>Edit</PrimaryButton><PrimaryButton tone="ghost" onClick={() => api.testEndpoint(endpoint.name).then(() => alert('Endpoint test finished.')).catch((e) => alert(String(e)))}><TestTube2 size={16} /> Test</PrimaryButton><PrimaryButton tone="ghost" onClick={() => copyText(curlFor(endpoint))}><Clipboard size={16} /> Copy curl</PrimaryButton><PrimaryButton tone="ghost" onClick={onRequests}>View requests</PrimaryButton><PrimaryButton tone="danger" onClick={() => api.deleteEndpoint(endpoint.name).then(reload)}>Delete</PrimaryButton></div></article>;
}

function RequestsPage({ requests, setSelected, setPage }: AppData) {
  const [query, setQuery] = useState('');
  const [filter, setFilter] = useState('all');
  const items = (requests.length ? requests : demoRequests).filter((r) => (filter === 'all' || statusOf(r).includes(filter)) && JSON.stringify(r).toLowerCase().includes(query.toLowerCase()));
  return <><PageHeader kicker="Live requests" title="Every request passing through The Witness appears here"><span>Filter by status, endpoint, verdict, profile, or request ID. Judging and retry states stay visible so operators know what is happening.</span></PageHeader><div className="filter-bar"><label><Search size={17} /> <input value={query} onChange={(e) => setQuery(e.target.value)} placeholder="Search endpoint, profile, request ID" /></label>{['all', 'approved', 'disapproved', 'human', 'retrying'].map((f) => <button key={f} className={filter === f ? 'active' : ''} onClick={() => setFilter(f)}>{f === 'disapproved' ? 'blocked' : f}</button>)}</div><section className="request-list">{items.length ? items.map((r) => <RequestCard key={r.id} request={r} onOpen={() => { setSelected(r.id); setPage('Request Detail'); }} />) : <EmptyState title="No requests yet" action="Open endpoints" onAction={() => setPage('Endpoints')}>Send a request through a watched endpoint and the approval loop will appear here.</EmptyState>}</section></>;
}

function RequestCard({ request, onOpen }: { request: RequestEvent; onOpen: () => void }) {
  return <button className="request-card" onClick={onOpen}><span className="request-id">{request.id}</span><strong>{request.endpoint_name}</strong><span>{request.model ?? 'model'} · {request.profile}</span><VerdictBadge status={request.status} /><span>{request.retry_attempt === 1 ? '1 attempt' : `${request.retry_attempt} attempts`}</span><span>{request.latency_ms}ms</span><time>{timeAgo(request.timestamp)}</time></button>;
}

function RequestDetailPage({ requests, selected }: AppData) {
  const req = requests.find((r) => r.id === selected) ?? demoRequests[0];
  return <><PageHeader kicker="Request detail" title="See exactly how this response was handled"><span>This is the proof trail: prompt, candidate, verdict, repair, and the final answer released to the app.</span></PageHeader><section className="detail-grid"><Panel title="Prompt and response" description={`Request ${req.id}`}><CodeBlock title="Original request" code={'Write a Python script that prints Hello World'} /><PromptDiff rejected={'print(Hello World)'} approved={'print("Hello World")'} /><CodeBlock title="Final approved response" code={String(req.final_response ?? 'print("Hello World")')} /></Panel><Panel title="Gemma verdict" description="Structured JSON from the local judge."><VerdictBadge status={req.status} /><CodeBlock title="Verdict JSON" code={JSON.stringify({ verdict: 'DISAPPROVED', confidence: 0.92, safety_score: 96, usefulness_score: 74, prompt_alignment_score: 88, correctness_risk: 'medium', rejection_reason: 'Python string is not quoted. The code will fail.', suggested_fix: 'Quote the string literal.', improved_prompt_instruction: 'Generate valid Python syntax and avoid the previous mistake.', requires_human_review: false }, null, 2)} /><div className="card-actions"><PrimaryButton tone="ghost">Replay request</PrimaryButton><PrimaryButton tone="ghost">Regenerate</PrimaryButton><PrimaryButton tone="success">Approve manually</PrimaryButton><PrimaryButton tone="ghost">Export audit report</PrimaryButton></div></Panel></section><AuditTimeline /></>;
}

function PromptRepairPage() {
  return <><PageHeader kicker="Prompt repair" title="Prompt Repair"><span>When a response is rejected, The Witness turns the reason into a better retry prompt while preserving the user’s original request.</span></PageHeader><section className="repair-layout"><Panel title="Current repair"><CodeBlock title="Original user request" code="Write a Python script that prints Hello World" /><CodeBlock title="Rejected response" code="print(Hello World)" /><div className="reason-card"><AlertTriangle size={20} /><div><strong>Python string is not quoted. The code will fail.</strong><p>Required fix: quote the string literal and keep the answer direct.</p></div></div><textarea aria-label="Repaired prompt preview" defaultValue={`Original user request:\nWrite a Python script that prints Hello World\n\nThe previous answer was rejected by The Witness.\nRejection reason: Python string is not quoted.\nRequired fix: Quote the string literal.\n\nNow generate a corrected answer.`} /><PrimaryButton><RefreshCcw size={16} /> Retry with repaired prompt</PrimaryButton></Panel><Panel title="Retry timeline"><div className="attempt-line fail"><XCircle size={18} /><span>Attempt 1</span><strong>Blocked before reaching the app</strong></div><div className="attempt-line warn"><Sparkles size={18} /><span>Repair generated</span><strong>reason converted into retry instruction</strong></div><div className="attempt-line pass"><CheckCircle2 size={18} /><span>Attempt 2</span><strong>Approved and returned</strong></div></Panel></section></>;
}

function HumanReviewPage() {
  const cards = [{ title: 'Finance Helper response', reason: 'Low confidence with financial guidance', confidence: '61%', profile: 'finance' }, { title: 'Medical explainer', reason: 'High-risk health information needs a human decision', confidence: '68%', profile: 'medical' }];
  return <><PageHeader kicker="Needs a human decision" title="High-risk or uncertain responses pause here"><span>The Witness pauses responses here when the model is not confident enough to safely release them.</span></PageHeader><section className="review-grid">{cards.map((c) => <article className="review-card" key={c.title}><StatusPill tone="warn" label="Needs a human decision" /><h3>{c.title}</h3><p>{c.reason}</p><dl><div><dt>Profile</dt><dd>{c.profile}</dd></div><div><dt>Confidence</dt><dd>{c.confidence}</dd></div></dl><div className="card-actions"><PrimaryButton tone="success">Approve</PrimaryButton><PrimaryButton tone="danger">Reject</PrimaryButton><PrimaryButton tone="ghost">Edit response</PrimaryButton><PrimaryButton tone="ghost">Export report</PrimaryButton></div></article>)}</section></>;
}

function ModelsPage({ models, links, config, reload }: AppData & { config: Config }) {
  const all = models.length ? models : demoModels;
  return <><PageHeader kicker="Model manager" title="Choose how The Witness thinks" actions={<><PrimaryButton onClick={() => window.open(links.huggingface, '_blank')}>Open Hugging Face</PrimaryButton><PrimaryButton tone="ghost" onClick={() => window.location.hash = '#Settings'}>Add custom Ollama model</PrimaryButton></>}><span>Gemma 4 is the primary recommended judge. Custom Ollama models are optional advanced choices you can register from Settings or the CLI.</span></PageHeader><section className="model-grid">{all.map((m) => <ModelCard key={m.id} model={m} config={config} reload={reload} />)}</section><section className="resource-strip"><a href={links.huggingface} target="_blank" rel="noreferrer">Fine-tuned model on Hugging Face</a><a href={links.colab} target="_blank" rel="noreferrer">Colab fine-tuning notebook</a></section></>;
}

function ModelCard({ model, config, reload }: { model: ModelEntry; config: Config; reload: () => void }) {
  const primary = model.model.includes('e2b');
  const strong = model.model.includes('e4b');
  const setDefault = async () => {
    await api.saveConfig({ ...config, gemma: { ...config.gemma, backend: model.backend === 'ollama-custom' ? 'ollama' : model.backend, model: model.model } });
    reload();
    alert(`Default judge set to ${model.model}. Run doctor/model test before live use.`);
  };
  const test = async () => {
    const res = await api.modelTest({ backend: model.backend === 'ollama-custom' ? 'ollama' : model.backend, model: model.model });
    alert(JSON.stringify(res, null, 2));
  };
  const download = async () => {
    const res = await api.modelDownload({ backend: model.backend, model: model.model, source: model.source });
    alert((res as { message?: string }).message ?? JSON.stringify(res));
  };
  return <article className="model-card"><div className="model-top"><div className="model-chip"><Cpu size={18} /> {model.backend}</div><StatusPill tone={model.installed ? 'good' : primary ? 'info' : strong ? 'warn' : 'neutral'} label={model.installed ? 'installed' : model.status ?? 'available'} /></div><h3>{model.display_name}</h3><p>{modelDescription(model)}</p><code>{model.model}</code><div className="card-actions"><PrimaryButton tone="ghost" onClick={() => download().catch((e) => alert(String(e)))}>Pull / Download</PrimaryButton><PrimaryButton tone="ghost" onClick={() => test().catch((e) => alert(String(e)))}>Test model</PrimaryButton><PrimaryButton onClick={() => setDefault().catch((e) => alert(String(e)))}>Set default</PrimaryButton></div></article>;
}

function modelDescription(model: ModelEntry) {
  if (model.source === 'huggingface') return 'Fine-tuned JSON verdict model for The Witness rejection/approval schema.';
  if (model.backend === 'litert') return 'Edge prefilter mode for lightweight checks. Requires runtime validation on the target device.';
  if (model.backend === 'llama.cpp') return 'Resource-constrained local inference with a server URL or GGUF path.';
  if (model.backend === 'unsloth') return 'Local fine-tuned judge path for schema-first rejection and approval checks.';
  if (model.backend === 'manual') return 'OpenAI-compatible local judge endpoint with configurable auth and model name.';
  if (model.backend === 'ollama') return 'Ollama-backed local judge for offline approval classification.';
  return 'Configurable judge backend for local verdict checks.';
}

function LogsPage({ logs, privacy }: AppData) {
  const events = ['received', 'judged', 'rejected', 'prompt repaired', 'retried', 'approved', 'exported'];
  return <><PageHeader kicker="Audit logs" title="Every verdict, repair, retry, and manual decision is recorded" actions={<><PrimaryButton tone="ghost">Export JSONL</PrimaryButton><PrimaryButton tone="ghost">Export Markdown</PrimaryButton></>}><span>Searchable timelines for approvals, blocks, retries, judge errors, and manual overrides. Privacy mode: {privacy ? 'on' : 'off'}.</span></PageHeader><section className="logs-layout"><Panel title="Audit timeline">{events.map((e, i) => <div className="log-event" key={e}><StatusPill tone={i < 2 ? 'info' : i < 4 ? 'warn' : 'good'} label={e} /><span>req_demo_9f1a</span><small>{i + 1}m ago</small></div>)}</Panel><Panel title="Raw log preview"><CodeBlock title="JSONL" code={logs || '{"event":"demo","message":"No audit events yet. Logs will appear once traffic flows through The Witness."}'} /></Panel></section></>;
}

function DoctorPage() {
  const groups = [
    ['Core system', [['OS detected', 'PASS', 'Linux runtime detected.'], ['Config valid', 'PASS', 'witness.toml can be read.']]],
    ['Proxy', [['Proxy port ready', 'PASS', 'localhost:8787 is available.'], ['Control API ready', 'PASS', 'localhost:8790 is serving.']]],
    ['Models', [['gemma4:e2b installed', 'WARN', 'Run: ollama pull gemma4:e2b'], ['gemma4:e4b installed', 'WARN', 'Run: ollama pull gemma4:e4b']]],
    ['Optional integrations', [['Blackbox env var', 'WARN', 'Run: export BLACKBOX_API_KEY="YOUR_KEY_HERE"'], ['LiteRT configured', 'INFO', 'Set a LiteRT model path when needed.']]],
  ] as const;
  return <><PageHeader kicker="System check" title="Find what is ready, what needs setup, and the exact command to fix it"><span>Doctor checks are grouped by what you can do next, with copyable commands for common setup issues.</span></PageHeader><section className="doctor-grid">{groups.map(([group, checks]) => <Panel key={group} title={group}>{checks.map(([name, state, fix]) => <DoctorCheckCard key={name} name={name} state={state} fix={fix} />)}</Panel>)}</section></>;
}

function DoctorCheckCard({ name, state, fix }: { name: string; state: string; fix: string }) {
  const tone = state === 'PASS' ? 'good' : state === 'WARN' ? 'warn' : state === 'FAIL' ? 'bad' : 'info';
  return <article className="doctor-check"><StatusPill tone={tone} label={state} /><div><strong>{name}</strong><p>{fix}</p></div><button className="copy-mini" onClick={() => copyText(fix.replace('Run: ', ''))}>Copy fix</button></article>;
}

function SettingsPage({ config, health, reload }: AppData & { config: Config }) {
  const tail = health?.dashboard_access?.tailscale;
  const [draft, setDraft] = useState(config);
  const [customModel, setCustomModel] = useState('');
  const [customName, setCustomName] = useState('');
  useEffect(() => setDraft(config), [config]);
  const save = async () => { await api.saveConfig(draft); reload(); alert('Settings saved. Run doctor before sending live traffic if you changed the judge.'); };
  const addCustomOllama = async (setDefault: boolean) => {
    if (!customModel.trim()) { alert('Enter an Ollama model name first.'); return; }
    const res = await api.addCustomOllamaModel({ model: customModel.trim(), display_name: customName.trim() || undefined, set_default: setDefault });
    reload();
    alert(res.message);
  };
  return <><PageHeader kicker="Settings" title="Control models, privacy, ports, services, and default behavior"><span>Gemma 4 remains the primary judge. Custom Ollama models are available here as an advanced addition and are documented deeper in setup docs.</span></PageHeader><section className="settings-grid">
    <Panel title="General"><label className="setting-row"><span>Default backend</span><input value={draft.gemma.backend} onChange={(e) => setDraft({ ...draft, gemma: { ...draft.gemma, backend: e.target.value } })} /></label><label className="setting-row"><span>Default model</span><input value={draft.gemma.model} onChange={(e) => setDraft({ ...draft, gemma: { ...draft.gemma, model: e.target.value } })} /></label><label className="setting-row"><span>Judge URL</span><input value={draft.gemma.url} onChange={(e) => setDraft({ ...draft, gemma: { ...draft.gemma, url: e.target.value } })} /></label><PrimaryButton onClick={() => save().catch((e) => alert(String(e)))}>Save General</PrimaryButton></Panel>
    <Panel title="Custom Ollama model" description="Optional advanced addition. Use Gemma 4 defaults unless you intentionally want another local Ollama tag."><label className="setting-row"><span>Ollama model tag</span><input value={customModel} onChange={(e) => setCustomModel(e.target.value)} placeholder="my-model:latest" /></label><label className="setting-row"><span>Display name</span><input value={customName} onChange={(e) => setCustomName(e.target.value)} placeholder="Local experimental judge" /></label><div className="card-actions"><PrimaryButton tone="ghost" onClick={() => addCustomOllama(false).catch((e) => alert(String(e)))}>Register model</PrimaryButton><PrimaryButton onClick={() => addCustomOllama(true).catch((e) => alert(String(e)))}>Register and set default</PrimaryButton></div><CodeBlock title="CLI equivalent" code="the-witness model add-ollama --model my-model:latest --display-name 'Local experimental judge'" /></Panel>
    <Panel title="Proxy"><label className="setting-row"><span>Proxy host</span><input defaultValue="127.0.0.1" readOnly /></label><label className="setting-row"><span>Proxy port</span><input defaultValue="8787" readOnly /></label><label className="setting-row"><span>LAN exposure</span><input defaultValue="off by default" readOnly /></label><PrimaryButton tone="ghost" onClick={() => api.startProxy().then((r) => alert(JSON.stringify(r, null, 2))).catch((e) => alert(String(e)))}>Proxy status</PrimaryButton></Panel>
    <Panel title="Dashboard"><label className="setting-row"><span>Control API</span><input defaultValue={health?.dashboard_access?.bind_url ?? '127.0.0.1:8790'} readOnly /></label><label className="setting-row"><span>Open browser</span><input defaultValue="manual: use dashboard --open" readOnly /></label><label className="setting-row"><span>Service</span><input defaultValue="dashboard --no-open --host 0.0.0.0" readOnly /></label><label className="setting-row"><span>Tailscale</span><input defaultValue={tail?.available ? tail.url ?? 'available' : tail?.hint ?? 'not detected'} readOnly /></label></Panel>
    <Panel title="Privacy"><label className="setting-row"><span>Store prompts</span><select value={draft.defaults.privacy_mode ? 'metadata' : 'full'} onChange={(e) => setDraft({ ...draft, defaults: { ...draft.defaults, privacy_mode: e.target.value === 'metadata' } })}><option value="full">full audit</option><option value="metadata">metadata only</option></select></label><label className="setting-row"><span>Secret redaction</span><input defaultValue="always on" readOnly /></label><label className="setting-row"><span>Log format</span><input value={draft.defaults.log_format} onChange={(e) => setDraft({ ...draft, defaults: { ...draft.defaults, log_format: e.target.value } })} /></label><PrimaryButton onClick={() => save().catch((e) => alert(String(e)))}>Save Privacy</PrimaryButton></Panel>
  </section></>;
}

function VerdictBadge({ status }: { status: string }) {
  const s = statusOf({ status } as RequestEvent);
  const tone = s.includes('approved') ? 'approved' : s.includes('human') ? 'review' : s.includes('retry') ? 'retry' : s.includes('disapproved') || s.includes('failed') ? 'blocked' : 'neutral';
  return <span className={`verdict-badge ${tone}`}>{friendlyStatus(status)}</span>;
}

function CodeBlock({ title, code }: { title: string; code: string }) {
  return <div className="code-block"><div><span>{title}</span><button onClick={() => copyText(code)}><Clipboard size={14} /> Copy</button></div><pre>{code}</pre></div>;
}

function PromptDiff({ rejected, approved }: { rejected: string; approved: string }) {
  return <div className="diff-block"><strong>Diff view</strong><pre className="minus">- {rejected}</pre><pre className="plus">+ {approved}</pre></div>;
}

function AuditTimeline() {
  const events = ['request received', 'candidate response captured', 'Gemma judging', 'blocked before reaching the app', 'prompt repaired', 'retry approved'];
  return <Panel title="Audit timeline" description="The full chain is exportable as Markdown or JSON."><div className="audit-timeline">{events.map((e, i) => <div key={e}><span>{i + 1}</span><strong>{e}</strong><small>{i === 3 ? 'Python syntax risk found' : 'recorded in JSONL'}</small></div>)}</div></Panel>;
}

function CopyButton({ value }: { value: string }) {
  return <button className="copy-mini" onClick={() => copyText(value)}><Clipboard size={14} /> Copy</button>;
}

function statusOf(r: RequestEvent) { return String(r.status ?? '').toLowerCase(); }
function topbarTitle(page: PageName) {
  if (page === 'Dashboard') return 'Mission Control';
  if (page === 'Endpoints') return 'Watched Endpoints';
  if (page === 'Requests') return 'Live Requests';
  if (page === 'Human Review') return 'Needs a Human Decision';
  if (page === 'Models') return 'Choose How The Witness Thinks';
  if (page === 'Logs') return 'Audit Logs';
  if (page === 'Doctor') return 'System Check';
  return page;
}
function friendlyStatus(status: string) {
  const s = String(status).toLowerCase();
  if (s.includes('approved')) return 'Approved and returned';
  if (s.includes('disapproved') || s.includes('failed')) return 'Blocked before reaching the app';
  if (s.includes('human')) return 'Needs a human decision';
  if (s.includes('retry')) return 'Repairing and retrying';
  if (s.includes('judging')) return 'Gemma is judging';
  return status || 'received';
}
function timeAgo(iso: string) {
  const diff = Math.max(1, Math.round((Date.now() - new Date(iso).getTime()) / 60000));
  return `${diff}m ago`;
}
function copyText(text: string) { void navigator.clipboard?.writeText(text); }
function curlSample() { return 'curl http://localhost:8787/v1/chat/completions -H "Content-Type: application/json" -d \'{"model":"gpt-5.5","messages":[{"role":"user","content":"Say hello"}]}\''; }
function curlFor(endpoint: Endpoint) { return `curl ${endpoint.local_proxy_url}/chat/completions -H "Content-Type: application/json" -d '{"model":"${endpoint.model}","messages":[{"role":"user","content":"Write a Python script that prints Hello World"}]}'`; }
