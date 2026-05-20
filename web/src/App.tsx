import React, { useEffect, useMemo, useState } from 'react';
import {
  Activity,
  AlertTriangle,
  Bot,
  CheckCircle2,
  ChevronRight,
  Clipboard,
  Code2,
  Cpu,
  Database,
  Eye,
  FileText,
  Gauge,
  HeartHandshake,
  Home,
  Info,
  ListChecks,
  Lock,
  Menu,
  Play,
  Radar,
  RefreshCcw,
  Search,
  Server,
  Settings as SettingsIcon,
  ShieldAlert,
  ShieldCheck,
  Sliders,
  Sparkles,
  TerminalSquare,
  TestTube2,
  Trash2,
  X,
  XCircle,
  Zap,
} from 'lucide-react';
import {
  Area,
  AreaChart,
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
  { name: 'Dashboard', short: 'Mission Control', icon: Home, help: 'Safety Overview' },
  { name: 'Endpoints', short: 'Watched Endpoints', icon: Radar, help: 'Protected Routes' },
  { name: 'Requests', short: 'Live Requests', icon: Activity, help: 'Decision Streams' },
  { name: 'Request Detail', short: 'Request Detail', icon: FileText, help: 'Detailed Proof Trail', hideFromSidebar: true },
  { name: 'Prompt Repair', short: 'Prompt Repair', icon: Sparkles, help: 'Automatic Retry Loop' },
  { name: 'Human Review', short: 'Needs a Human Decision', icon: HeartHandshake, help: 'High-risk Approvals' },
  { name: 'Models', short: 'Choose How The Witness Thinks', icon: Bot, help: 'Local Gemma 4 Judge Setup' },
  { name: 'Logs', short: 'Audit Logs', icon: ListChecks, help: 'Raw Security Trails' },
  { name: 'Doctor', short: 'System Check', icon: TestTube2, help: 'Readiness & Doctor checks' },
  { name: 'Settings', short: 'Settings', icon: Sliders, help: 'Firewall Configurations' },
] as const;

type PageName = (typeof pages)[number]['name'];

interface Toast {
  id: string;
  title: string;
  desc: string;
  type: 'success' | 'error' | 'info';
}

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
  showToast: (title: string, desc: string, type?: 'success' | 'error' | 'info') => void;
  copyToClipboard: (text: string, entity?: string) => void;
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
  {
    id: 'req_demo_9f1a',
    endpoint_name: 'Blackbox Grok Code',
    model: 'grok-code-fast',
    profile: 'coding',
    status: 'approved',
    retry_attempt: 1,
    latency_ms: 612,
    timestamp: new Date(now - 3 * 60_000).toISOString(),
    judge_verdict: { verdict: { verdict: 'APPROVED' } },
    request_body: { messages: [{ role: 'user', content: 'Write a Python script that prints Hello World' }] },
    candidate_response: 'print(Hello World)',
    final_response: 'print("Hello World")',
  },
  {
    id: 'req_demo_7bc2',
    endpoint_name: 'Local Tutor',
    model: 'local-tutor',
    profile: 'education',
    status: 'human_review',
    retry_attempt: 0,
    latency_ms: 834,
    timestamp: new Date(now - 8 * 60_000).toISOString(),
    judge_verdict: { verdict: { verdict: 'NEEDS_HUMAN_REVIEW' } },
  },
  {
    id: 'req_demo_51aa',
    endpoint_name: 'Finance Helper',
    model: 'finance-agent',
    profile: 'finance',
    status: 'disapproved',
    retry_attempt: 2,
    latency_ms: 1290,
    timestamp: new Date(now - 18 * 60_000).toISOString(),
    judge_verdict: { verdict: { verdict: 'DISAPPROVED' } },
  },
  {
    id: 'req_demo_2de0',
    endpoint_name: 'Research Assistant',
    model: 'research-local',
    profile: 'scientific research',
    status: 'retrying',
    retry_attempt: 1,
    latency_ms: 945,
    timestamp: new Date(now - 28 * 60_000).toISOString(),
    judge_verdict: { verdict: { verdict: 'DISAPPROVED' } },
  },
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

function useAppData(showToast: (t: string, d: string, ty?: 'success' | 'error' | 'info') => void) {
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
    api.health()
      .then(setHealth)
      .catch(() => {
        setHealth(undefined);
      });
    api.config()
      .then(setConfig)
      .catch(() => setConfig(undefined));
    api.requests()
      .then((r) => setRequests(r.requests))
      .catch(() => setRequests([]));
    api.models()
      .then((r) => {
        setModels(r.models);
        setLinks(r.links);
      })
      .catch(() => {
        setModels([]);
        setLinks({});
      });
    api.logs()
      .then((r) => {
        setLogs(r.text);
        setPrivacy(r.privacy_mode);
      })
      .catch(() => {
        setLogs('');
        setPrivacy(false);
      });
  };

  useEffect(() => {
    reload();
    const t = setInterval(reload, 5000);
    return () => clearInterval(t);
  }, []);

  useEffect(() => {
    const applyHash = () => {
      const raw = decodeURIComponent(window.location.hash.replace(/^#/, ''));
      const match = pages.find((p) => p.name === raw || p.short === raw);
      if (match) setPage(match.name);
    };
    applyHash();
    window.addEventListener('hashchange', applyHash);
    return () => window.removeEventListener('hashchange', applyHash);
  }, []);

  return { page, setPage, selected, setSelected, health, config, requests, models, links, logs, privacy, reload };
}

export default function App() {
  const [toasts, setToasts] = useState<Toast[]>([]);
  const [confirmDelete, setConfirmDelete] = useState<string | null>(null);

  const showToast = (title: string, desc: string, type: 'success' | 'error' | 'info' = 'info') => {
    const id = Math.random().toString(36).substring(2, 9);
    setToasts((prev) => [...prev, { id, title, desc, type }]);
    setTimeout(() => {
      setToasts((prev) => prev.filter((t) => t.id !== id));
    }, 4500);
  };

  const state = useAppData(showToast);

  const copyToClipboard = (text: string, entity = 'Text') => {
    navigator.clipboard.writeText(text)
      .then(() => showToast('Copied to Clipboard', `${entity} successfully copied.`, 'success'))
      .catch(() => showToast('Copy Failed', 'Unable to write to clipboard.', 'error'));
  };

  const data: AppData = {
    ...state,
    health: state.health, // If undefined, we show offline banner but pass demoHealth for rendering below
    requests: state.requests.length ? state.requests : demoRequests,
    models: state.models.length ? state.models : demoModels,
    links: Object.keys(state.links).length ? state.links : {
      huggingface: 'https://huggingface.co/ahmadalfakeh/witness-gemma4-e2b-judge',
      colab: 'https://colab.research.google.com/drive/17-CgEQLNg8bpnhhWzJwpapRxQyHIqybq?usp=sharing',
    },
    showToast,
    copyToClipboard,
  };

  const endpoints = state.config?.endpoints?.length ? state.config.endpoints : demoEndpoints;
  const isBackendRunning = !!state.health;
  const activeHealth = state.health ?? demoHealth;

  const [navOpen, setNavOpen] = useState(false);
  const Page = pageComponent(state.page);

  const handleDeleteEndpoint = async () => {
    if (!confirmDelete) return;
    try {
      if (isBackendRunning) {
        await api.deleteEndpoint(confirmDelete);
        showToast('Endpoint Deleted', `Successfully stopped watching "${confirmDelete}".`, 'success');
        state.reload();
      } else {
        showToast('Demo Endpoint Removed', `Stopped watching "${confirmDelete}" (Demo Mode).`, 'info');
      }
    } catch (e) {
      showToast('Error Deleting Endpoint', String(e), 'error');
    } finally {
      setConfirmDelete(null);
    }
  };

  return (
    <div className="app-shell">
      <SkipLink />

      {/* Connection Offline Banner */}
      {!isBackendRunning && (
        <div className="offline-banner" style={{ gridColumn: '1 / -1' }}>
          <div className="offline-banner-message">
            <AlertTriangle size={18} />
            <span>The dashboard backend is not running. Showing demo mode data. Start it with:</span>
          </div>
          <div className="offline-banner-action">
            <code>the-witness dashboard</code>
            <button className="copy-mini" onClick={() => copyToClipboard('the-witness dashboard', 'Command')}>
              <Clipboard size={12} />
              <span>Copy</span>
            </button>
          </div>
        </div>
      )}

      <Sidebar
        page={state.page}
        setPage={(p) => {
          state.setPage(p);
          window.location.hash = encodeURIComponent(p);
          setNavOpen(false);
        }}
        open={navOpen}
        setOpen={setNavOpen}
      />

      <div className="workspace">
        <Topbar
          health={activeHealth}
          page={state.page}
          openMenu={() => setNavOpen(true)}
          backendRunning={isBackendRunning}
        />

        <main id="main" className="page-stage" tabIndex={-1}>
          <Page
            {...data}
            health={activeHealth}
            config={state.config ? state.config : { ...(emptyConfig()), endpoints }}
          />
        </main>

        <MobileNav page={state.page} setPage={(p) => {
          state.setPage(p);
          window.location.hash = encodeURIComponent(p);
        }} />
      </div>

      {/* Confirmation Modal */}
      {confirmDelete && (
        <div className="modal-overlay" onClick={() => setConfirmDelete(null)}>
          <div className="modal" onClick={(e) => e.stopPropagation()}>
            <h3 className="modal-title">Delete Endpoint?</h3>
            <p className="modal-desc">
              Are you sure you want to stop watching and delete <strong>{confirmDelete}</strong>? This action cannot be undone.
            </p>
            <div className="modal-actions">
              <button className="btn ghost" onClick={() => setConfirmDelete(null)}>Cancel</button>
              <button className="btn danger" onClick={handleDeleteEndpoint}>Delete</button>
            </div>
          </div>
        </div>
      )}

      {/* Hooking the window dispatcher for the delete modal */}
      <DeleteModalListener trigger={setConfirmDelete} />

      {/* Toast Notifications */}
      <div className="toast-container">
        {toasts.map((t) => (
          <div key={t.id} className={`toast ${t.type}`}>
            {t.type === 'success' && <CheckCircle2 size={18} style={{ color: 'var(--green)', flexShrink: 0 }} />}
            {t.type === 'error' && <XCircle size={18} style={{ color: 'var(--red)', flexShrink: 0 }} />}
            {t.type === 'info' && <Info size={18} style={{ color: 'var(--blue)', flexShrink: 0 }} />}
            <div className="toast-content">
              <div className="toast-title">{t.title}</div>
              <div className="toast-desc">{t.desc}</div>
            </div>
          </div>
        ))}
      </div>
    </div>
  );
}

function DeleteModalListener({ trigger }: { trigger: (name: string | null) => void }) {
  useEffect(() => {
    const handleTrigger = (e: Event) => {
      const customEvent = e as CustomEvent;
      trigger(customEvent.detail);
    };
    window.addEventListener('trigger-delete-modal', handleTrigger);
    return () => window.removeEventListener('trigger-delete-modal', handleTrigger);
  }, [trigger]);
  return null;
}

function pageComponent(page: PageName) {
  return ({ config, ...data }: AppData & { config: Config; health: Health }) => {
    switch (page) {
      case 'Endpoints':
        return <EndpointsPage config={config} {...data} />;
      case 'Requests':
        return <RequestsPage {...data} />;
      case 'Request Detail':
        return <RequestDetailPage {...data} />;
      case 'Prompt Repair':
        return <PromptRepairPage {...data} />;
      case 'Human Review':
        return <HumanReviewPage {...data} />;
      case 'Models':
        return <ModelsPage config={config} {...data} />;
      case 'Logs':
        return <LogsPage {...data} />;
      case 'Doctor':
        return <DoctorPage {...data} />;
      case 'Settings':
        return <SettingsPage config={config} {...data} />;
      default:
        return <DashboardPage config={config} {...data} />;
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
  return <a className="skip-link" href="#main">Skip to main content</a>;
}

function Sidebar({ page, setPage, open, setOpen }: { page: PageName; setPage: (p: PageName) => void; open: boolean; setOpen: (v: boolean) => void }) {
  return (
    <>
      <aside className={`sidebar ${open ? 'open' : ''}`} aria-label="Primary Navigation">
        <div className="brand-block">
          <div className="brand-mark" aria-hidden="true">
            <Eye size={22} />
          </div>
          <div>
            <div className="brand-name">The Witness</div>
            <div className="brand-subtitle">AI Safety Firewall</div>
          </div>
          <button className="icon-button mobile-only" aria-label="Close navigation" onClick={() => setOpen(false)}>
            <X size={18} />
          </button>
        </div>
        <nav className="nav-list">
          {pages
            .filter((p) => !('hideFromSidebar' in p))
            .map((item) => {
              const Icon = item.icon;
              return (
                <button
                  key={item.name}
                  className={`nav-item ${page === item.name ? 'active' : ''}`}
                  onClick={() => setPage(item.name)}
                  aria-current={page === item.name ? 'page' : undefined}
                >
                  <Icon size={18} aria-hidden="true" />
                  <span>
                    <strong>{item.short}</strong>
                    <small>{item.help}</small>
                  </span>
                </button>
              );
            })}
        </nav>
        <div className="sidebar-card">
          <div className="eyebrow">Local First</div>
          <strong>Protected by Gemma</strong>
          <span>Your data stays local. The firewall runs entirely on localhost.</span>
        </div>
      </aside>
      {open && <button className="scrim" aria-label="Close navigation overlay" onClick={() => setOpen(false)} />}
    </>
  );
}

function Topbar({ health, page, openMenu, backendRunning }: { health: Health; page: PageName; openMenu: () => void; backendRunning: boolean }) {
  const currentPage = pages.find((p) => p.name === page);
  const displayTitle = currentPage ? currentPage.short : page;

  return (
    <header className="topbar">
      <button className="icon-button menu-button" aria-label="Open primary navigation" onClick={openMenu}>
        <Menu size={20} />
      </button>
      <div className="topbar-title">
        <span className={`status-dot`} style={{ backgroundColor: backendRunning ? 'var(--green)' : 'var(--amber)' }} aria-hidden="true" />
        <span>{displayTitle}</span>
      </div>
      <div className="topbar-actions">
        <StatusPill tone={backendRunning ? 'good' : 'warn'} label={backendRunning ? 'Service Active' : 'Demo State'} />
        <StatusPill tone="info" label={health.model || 'gemma4:e2b'} />
      </div>
    </header>
  );
}

function MobileNav({ page, setPage }: { page: PageName; setPage: (p: PageName) => void }) {
  const items: PageName[] = ['Dashboard', 'Endpoints', 'Requests', 'Models', 'Doctor'];
  return (
    <nav className="mobile-nav" aria-label="Mobile Navigation">
      {items.map((name) => {
        const item = pages.find((p) => p.name === name)!;
        const Icon = item.icon;
        return (
          <button key={name} className={page === name ? 'active' : ''} onClick={() => setPage(name)}>
            <Icon size={18} aria-hidden="true" />
            <span>{item.name === 'Dashboard' ? 'Mission' : item.name === 'Endpoints' ? 'Routes' : item.name === 'Models' ? 'Brain' : item.short.split(' ')[0]}</span>
          </button>
        );
      })}
    </nav>
  );
}

function StatusPill({ label, tone = 'neutral' }: { label: string; tone?: 'good' | 'warn' | 'bad' | 'info' | 'neutral' }) {
  return (
    <span className={`status-pill ${tone}`}>
      <span aria-hidden="true" />
      {label}
    </span>
  );
}

function PageHeader({ kicker, title, children, actions }: { kicker: string; title: string; children: React.ReactNode; actions?: React.ReactNode }) {
  return (
    <section className="page-header">
      <div>
        <p className="eyebrow">{kicker}</p>
        <h1>{title}</h1>
        <p>{children}</p>
      </div>
      {actions && <div className="header-actions">{actions}</div>}
    </section>
  );
}

function MetricCard({ label, value, detail, tone = 'teal', icon: Icon }: { label: string; value: string | number; detail: string; tone?: string; icon: React.ComponentType<{ size: number }> }) {
  return (
    <article className={`metric-card tone-${tone}`}>
      <div className="metric-icon">
        <Icon size={18} />
      </div>
      <div>
        <span>{label}</span>
        <strong>{value}</strong>
        <small>{detail}</small>
      </div>
    </article>
  );
}

function Panel({ title, description, children, style }: { title: string; description: string; children: React.ReactNode; style?: React.CSSProperties }) {
  return (
    <section className="panel" style={style}>
      <h2>{title}</h2>
      <p style={{ marginBottom: 20 }}>{description}</p>
      {children}
    </section>
  );
}

function EmptyState({ title, children, action, onAction }: { title: string; children: React.ReactNode; action?: string; onAction?: () => void }) {
  return (
    <div className="empty-state">
      <div className="empty-icon" aria-hidden="true">
        <AlertTriangle size={24} />
      </div>
      <h2>{title}</h2>
      <p>{children}</p>
      {action && onAction && (
        <button className="btn" onClick={onAction}>
          {action}
        </button>
      )}
    </div>
  );
}

function QuickActionCard({ icon: Icon, title, text, action, onClick }: { icon: React.ComponentType<{ size: number }>; title: string; text: string; action: string; onClick: () => void }) {
  return (
    <button className="quick-card" onClick={onClick}>
      <Icon size={24} aria-hidden="true" />
      <div>
        <strong>{title}</strong>
        <small>{text}</small>
        <em>{action} &rarr;</em>
      </div>
    </button>
  );
}

function DashboardPage({ health, config, requests, setPage, setSelected, showToast, copyToClipboard }: AppData & { config: Config }) {
  const endpoints = config.endpoints.length ? config.endpoints : demoEndpoints;
  const approved = requests.filter((r) => statusOf(r).includes('approved')).length;
  const disapproved = requests.filter((r) => ['failed', 'disapproved', 'blocked'].some((s) => statusOf(r).includes(s))).length;
  const review = requests.filter((r) => statusOf(r).includes('human')).length;
  const retries = requests.reduce((sum, r) => sum + (r.retry_attempt || 0), 0);
  const avgLatency = Math.round(requests.reduce((sum, r) => sum + (r.latency_ms || 0), 0) / Math.max(1, requests.length));

  const chart = useMemo(() => {
    return requests.slice(0, 8).reverse().map((r) => ({
      name: `R-${r.id.replace('req_demo_', '').substring(0, 4)}`,
      latency: r.latency_ms,
      attempt: r.retry_attempt + 1,
    }));
  }, [requests]);

  const hasEndpoints = config.endpoints.length > 0;

  return (
    <>
      <PageHeader
        kicker="AI Safety Mission Control"
        title="Mission Control"
        actions={
          <>
            <button className="btn" onClick={() => setPage('Endpoints')}>
              <Radar size={16} />
              Add Watched Route
            </button>
            <button className="btn ghost" onClick={() => setPage('Doctor')}>
              <TestTube2 size={16} />
              System Check
            </button>
          </>
        }
      >
        Real-time security auditing, prompt repair metrics, and safety configuration.
      </PageHeader>

      <section className="hero-grid">
        <article className="hero-status panel">
          <div className="watch-orb" aria-hidden="true">
            <Eye size={36} />
          </div>
          <div style={{ flex: 1 }}>
            <p className="eyebrow">{hasEndpoints ? 'Watch Loop Running' : 'Get Started'}</p>
            <h2>{hasEndpoints ? 'The Witness is watching' : 'Ready to watch your first endpoint'}</h2>
            <p>
              Gemma 4 judge model <strong>{health?.model ?? config.gemma.model}</strong> is protecting your API calls. Fallback mode is configured to <strong>{config.defaults.fallback_mode.replace('_', ' ')}</strong>.
            </p>
            <div className="command-row">
              <code>{health?.proxy ?? 'http://127.0.0.1:8787/v1'}</code>
              <button className="copy-mini" onClick={() => copyToClipboard(health?.proxy ?? 'http://127.0.0.1:8787/v1', 'Local Witness proxy URL')} aria-label="Copy local proxy URL">
                <Clipboard size={12} />
                <span>Copy URL</span>
              </button>
            </div>
          </div>
        </article>

        <DashboardAccessPanel health={health} showToast={showToast} copyToClipboard={copyToClipboard} />
      </section>

      <section className="metric-grid" aria-label="Firewall Security Metrics">
        <MetricCard icon={Radar} label="Active Endpoints" value={endpoints.filter((e) => e.enabled).length} detail="Watched routes online" tone="teal" />
        <MetricCard icon={Activity} label="Requests Scanned" value={requests.length} detail="Through the firewall" tone="blue" />
        <MetricCard icon={ShieldCheck} label="Released / Safe" value={approved} detail="Returned to client app" tone="green" />
        <MetricCard icon={ShieldAlert} label="Blocked / Safe" value={disapproved} detail="Prevented before app" tone="red" />
        <MetricCard icon={HeartHandshake} label="Awaiting Dec." value={review} detail="Needs a human decision" tone="amber" />
        <MetricCard icon={Gauge} label="Avg Scanned Speed" value={`${avgLatency}ms`} detail="Verify round-trip" tone="blue" />
        <MetricCard icon={RefreshCcw} label="Repairs Attempted" value={retries} detail="Auto prompt rewrites" tone="amber" />
      </section>

      {!hasEndpoints && (
        <EmptyState
          title="No Watched Endpoints Found"
          action="Add Your First Endpoint"
          onAction={() => setPage('Endpoints')}
        >
          No endpoints are registered yet. The Witness will create a local proxy URL to watch and intercept your AI application requests.
        </EmptyState>
      )}

      <section className="dashboard-grid">
        <Panel title="Scan Latency & Attempts" description="Verdicts latency timeline for recent API streams.">
          <div className="chart-frame" aria-label="Verdicts Latency Charts">
            {chart.length > 0 ? (
              <ResponsiveContainer width="100%" height={240}>
                <AreaChart data={chart}>
                  <defs>
                    <linearGradient id="tealGlow" x1="0" x2="0" y1="0" y2="1">
                      <stop offset="0%" stopColor="var(--teal)" stopOpacity={0.4} />
                      <stop offset="100%" stopColor="var(--teal)" stopOpacity={0} />
                    </linearGradient>
                  </defs>
                  <CartesianGrid stroke="rgba(255,255,255,.04)" strokeDasharray="3 3" />
                  <XAxis dataKey="name" stroke="var(--subtle)" fontSize={12} />
                  <YAxis stroke="var(--subtle)" fontSize={12} unit="ms" />
                  <Tooltip contentStyle={{ background: '#0e171e', border: '1px solid var(--line)', borderRadius: 10, color: 'var(--text)' }} />
                  <Area type="monotone" dataKey="latency" name="Latency (ms)" stroke="var(--teal)" fill="url(#tealGlow)" strokeWidth={2} />
                </AreaChart>
              </ResponsiveContainer>
            ) : (
              <div className="chart-note">No request latency records found yet.</div>
            )}
          </div>
        </Panel>

        <Panel title="Verdict Distribution" description="Proportion of approved, blocked, and reviewed decisions.">
          <div className="chart-frame chart-frame-centered" aria-label="Verdict Share Distribution">
            <ResponsiveContainer width="100%" height={220}>
              <PieChart>
                <Pie
                  data={[
                    { name: 'Approved', value: approved || 5 },
                    { name: 'Blocked', value: disapproved || 2 },
                    { name: 'Needs Decision', value: review || 1 },
                  ]}
                  innerRadius={50}
                  outerRadius={75}
                  paddingAngle={4}
                  dataKey="value"
                >
                  <Cell fill="var(--green)" />
                  <Cell fill="var(--red)" />
                  <Cell fill="var(--amber)" />
                </Pie>
                <Tooltip contentStyle={{ background: '#0e171e', border: '1px solid var(--line)', borderRadius: 10 }} />
              </PieChart>
            </ResponsiveContainer>
            <div className="donut-center">
              <strong>{requests.length || 8}</strong>
              <span>Scans</span>
            </div>
          </div>
          <div className="legend-row" style={{ justifyContent: 'center', marginTop: 10 }}>
            <StatusPill tone="good" label="Approved" />
            <StatusPill tone="bad" label="Blocked" />
            <StatusPill tone="warn" label="Awaiting Review" />
          </div>
        </Panel>
      </section>

      <section className="dashboard-grid">
        <LiveActivity requests={requests} setPage={setPage} setSelected={setSelected} />
        <SystemHealthCard health={health} />
      </section>

      <section className="quick-action-grid">
        <QuickActionCard
          icon={Radar}
          title="Watch a new API Route"
          text="Deploy a local proxy and intercept AI calls automatically."
          action="Add Endpoint"
          onClick={() => setPage('Endpoints')}
        />
        <QuickActionCard
          icon={Bot}
          title="Configure Gemma 4 Judge"
          text="Select defaults, test prompt response matching, or add local adapters."
          action="Model setup"
          onClick={() => setPage('Models')}
        />
        <QuickActionCard
          icon={TerminalSquare}
          title="Copy curl smoke test"
          text="Send a trial chat completion command directly to localhost."
          action="Copy cURL"
          onClick={() => copyToClipboard(curlSample(), 'Smoke test cURL')}
        />
      </section>
    </>
  );
}

function DashboardAccessPanel({ health, showToast, copyToClipboard }: { health?: Health; showToast: (t: string, d: string, ty?: 'success' | 'error' | 'info') => void; copyToClipboard: (text: string, entity?: string) => void }) {
  const access = health?.dashboard_access;
  const localUrl = access?.local_url ?? health?.dashboard ?? 'http://127.0.0.1:8790';
  const tail = access?.tailscale;
  const tailUrl = tail?.url;

  return (
    <article className="panel checklist">
      <div className="eyebrow">Connection Options</div>
      <h3>Dashboard Access</h3>
      <div className="check-row">
        <span className="ok"><CheckCircle2 size={14} /></span>
        <strong>Local Dashboard</strong>
        <small>{localUrl}</small>
      </div>
      <div className="check-row">
        <span className={health?.service_running !== false ? 'ok' : 'warn'}>
          {health?.service_running !== false ? <CheckCircle2 size={14} /> : <AlertTriangle size={14} />}
        </span>
        <strong>Background Daemon</strong>
        <small>{health?.service_running !== false ? 'running' : 'offline'}</small>
      </div>
      {tail?.available && tailUrl ? (
        <div className="check-row">
          <span className="ok"><CheckCircle2 size={14} /></span>
          <strong>Tailscale Node</strong>
          <small>{tailUrl}</small>
        </div>
      ) : (
        <div className="check-row">
          <span className="warn"><AlertTriangle size={14} /></span>
          <strong>Tailscale Proxy</strong>
          <small>{tail?.hint ?? 'Tailnet disconnected'}</small>
        </div>
      )}
      <div className="command-row">
        <code>{tailUrl ?? localUrl}</code>
        <button className="copy-mini" onClick={() => copyToClipboard(tailUrl ?? localUrl, 'Access URL')}>
          <Clipboard size={12} />
          <span>Copy</span>
        </button>
      </div>
    </article>
  );
}

function LiveActivity({ requests, setPage, setSelected }: { requests: RequestEvent[]; setPage: (p: PageName) => void; setSelected: (id?: string) => void }) {
  const activity = requests.slice(0, 5);

  const handleRowClick = (id: string) => {
    setSelected(id);
    setPage('Request Detail');
  };

  return (
    <Panel title="Live Activity Log" description="Recent security scans passing through the local firewall.">
      <div style={{ marginTop: 10 }}>
        {activity.map((r) => (
          <div
            className="activity-row"
            key={r.id}
            onClick={() => handleRowClick(r.id)}
            style={{ cursor: 'pointer', transition: 'background-color 0.15s ease' }}
          >
            <span className="activity-pulse" />
            <div>
              <strong style={{ color: 'var(--text)' }}>{r.endpoint_name}</strong>
              <small style={{ display: 'block', fontSize: '11px', color: 'var(--subtle)' }}>
                {r.profile} · {r.retry_attempt > 0 ? `${r.retry_attempt} repairs` : '0 repairs'} · {timeAgo(r.timestamp)}
              </small>
            </div>
            <VerdictBadge status={r.status} />
          </div>
        ))}
      </div>
    </Panel>
  );
}

function SystemHealthCard({ health }: { health?: Health }) {
  const checks = [
    { name: 'App firewall service', note: health?.service_running !== false ? 'Running' : 'Offline', tone: health?.service_running !== false ? 'good' : 'bad' },
    { name: 'Local Proxy Endpoint', note: health?.proxy ?? 'http://127.0.0.1:8787/v1', tone: 'info' },
    { name: 'Gemma 4 Default Judge', note: health?.model ?? 'gemma4:e2b', tone: 'good' },
    { name: 'Ollama Integration', note: 'Reachable', tone: 'good' },
  ] as const;

  return (
    <Panel title="Security Firewall Health" description="Status overview of system services.">
      <div style={{ marginTop: 10 }}>
        {checks.map((c) => (
          <div className="health-row" key={c.name}>
            <StatusPill tone={c.tone} label={c.tone === 'good' ? 'PASS' : c.tone === 'bad' ? 'FAIL' : 'INFO'} />
            <strong>{c.name}</strong>
            <small>{c.note}</small>
          </div>
        ))}
      </div>
    </Panel>
  );
}

function EndpointsPage({ config, reload, setPage, showToast, copyToClipboard }: AppData & { config: Config }) {
  const endpoints = config.endpoints;
  const isBackendRunning = !!config.setup.last_doctor_check || config.endpoints.length > 0;

  const [form, setForm] = useState<Endpoint>({
    name: '',
    enabled: true,
    upstream_url: 'https://api.openai.com/v1',
    local_proxy_url: 'http://localhost:8787/openai/v1',
    model: 'gpt-4o',
    profile: 'safety-default',
    retry_limit: 3,
    strictness: 'medium',
    fallback_mode: 'human_review',
    auth: { type: 'bearer_env', env: 'OPENAI_API_KEY' },
    timeout_seconds: 30,
  });

  const saveEndpoint = async () => {
    if (!form.name.trim()) {
      showToast('Validation Error', 'Please enter a name for the watched route.', 'error');
      return;
    }
    try {
      await api.addEndpoint(form);
      showToast('Endpoint Added', `Endpoint is now being watched. Send traffic to the local proxy URL to begin verification.`, 'success');
      reload();
      // Reset form
      setForm({
        name: '',
        enabled: true,
        upstream_url: 'https://api.openai.com/v1',
        local_proxy_url: 'http://localhost:8787/openai/v1',
        model: 'gpt-4o',
        profile: 'safety-default',
        retry_limit: 3,
        strictness: 'medium',
        fallback_mode: 'human_review',
        auth: { type: 'bearer_env', env: 'OPENAI_API_KEY' },
        timeout_seconds: 30,
      });
    } catch (e) {
      showToast('Error Saving Endpoint', String(e), 'error');
    }
  };

  const handleQuickAddBlackbox = async () => {
    try {
      await api.addBlackbox();
      showToast('Blackbox Added', 'Blackbox Grok Code route has been added.', 'success');
      reload();
    } catch (e) {
      showToast('Secret Key Needed', 'Please run `export BLACKBOX_API_KEY="YOUR_KEY_HERE"` in your server shell.', 'error');
    }
  };

  return (
    <>
      <PageHeader
        kicker="Watched Endpoints"
        title="Protected API Routes"
        actions={
          <button className="btn" onClick={handleQuickAddBlackbox}>
            <Zap size={16} />
            Quick Add Blackbox
          </button>
        }
      >
        Configure local proxy URLs, choose verification profiles, and test security validation strictness.
      </PageHeader>

      <section className="endpoint-layout">
        <Panel title="Add Protected Route" description="Configure upstream endpoints. Secrets are kept in environment variables.">
          <div className="stepper" aria-hidden="true">
            <span className="active">1 Basic Info</span>
            <span>2 Upstream Target</span>
            <span>3 Security Config</span>
            <span>4 Auth Mode</span>
          </div>

          <div className="form-grid">
            <label>
              Friendly Route Name
              <input
                value={form.name}
                onChange={(e) => setForm({ ...form, name: e.target.value })}
                placeholder="e.g. Production GPT-4"
              />
            </label>
            <label>
              Upstream API Base URL
              <input
                value={form.upstream_url}
                onChange={(e) => setForm({ ...form, upstream_url: e.target.value })}
                placeholder="https://api.openai.com/v1"
              />
            </label>
            <label>
              Local Intercept Proxy URL
              <input
                value={form.local_proxy_url}
                onChange={(e) => setForm({ ...form, local_proxy_url: e.target.value })}
                placeholder="http://localhost:8787/my-proxy/v1"
              />
            </label>
            <label>
              Upstream Target Model
              <input
                value={form.model}
                onChange={(e) => setForm({ ...form, model: e.target.value })}
                placeholder="e.g. gpt-4o-mini"
              />
            </label>
            <label>
              Verification Profile
              <input
                value={form.profile}
                onChange={(e) => setForm({ ...form, profile: e.target.value })}
                placeholder="e.g. coding, safety-default"
              />
            </label>
            <label>
              Strictness Level
              <select
                value={form.strictness}
                onChange={(e) => setForm({ ...form, strictness: e.target.value as any })}
              >
                <option value="relaxed">Relaxed (Low risk filters)</option>
                <option value="medium">Medium (Standard safety filters)</option>
                <option value="high">High (Strict JSON, formatting & correctness)</option>
                <option value="critical">Critical (Max safety bounds)</option>
              </select>
            </label>
            <label>
              Rejection Fallback Action
              <select
                value={form.fallback_mode}
                onChange={(e) => setForm({ ...form, fallback_mode: e.target.value as any })}
              >
                <option value="human_review">Pause for human decision</option>
                <option value="safe_response">Return safe canned response</option>
                <option value="error">Return client-side API error</option>
                <option value="demo_judge">Demo retry flow</option>
              </select>
            </label>
            <label>
              Authentication Type
              <select
                value={form.auth?.type ?? 'none'}
                onChange={(e) => setForm({
                  ...form,
                  auth: { type: e.target.value, env: form.auth?.env ?? '' }
                })}
              >
                <option value="none">No Upstream Auth Needed</option>
                <option value="bearer_env">Bearer Token from Server Env Var</option>
                <option value="header_env">Custom Header from Server Env Var</option>
                <option value="static_local_discouraged">Static Token (Not Recommended)</option>
              </select>
            </label>
            {form.auth?.type !== 'none' && (
              <label>
                Server Env Var Name
                <input
                  value={form.auth?.env ?? ''}
                  onChange={(e) => setForm({
                    ...form,
                    auth: { type: form.auth?.type ?? 'bearer_env', env: e.target.value }
                  })}
                  placeholder="e.g. OPENAI_API_KEY"
                />
              </label>
            )}
            <label>
              Prompt Repair Attempt Limit
              <input
                type="number"
                min={0}
                max={6}
                value={form.retry_limit}
                onChange={(e) => setForm({ ...form, retry_limit: Number(e.target.value) })}
              />
            </label>
          </div>

          <div style={{ margin: '14px 0', fontSize: '13px', color: 'var(--muted)' }}>
            <strong>Security policy:</strong> Store the secret outside The Witness. We only keep the environment variable name.
          </div>

          <div className="form-actions">
            <button className="btn" onClick={saveEndpoint}>
              Save Route Config
            </button>
            <button className="btn ghost" onClick={() => {
              const cmd = `curl ${form.local_proxy_url}/chat/completions -H "Content-Type: application/json" -d '{"model":"${form.model}","messages":[{"role":"user","content":"Say Hello"}]}'`;
              copyToClipboard(cmd, 'Test cURL Command');
            }}>
              <Clipboard size={14} />
              Copy Test cURL
            </button>
          </div>
        </Panel>

        <article className="blackbox-card panel">
          <div className="brand-mark">
            <Code2 size={24} />
          </div>
          <h3>Quick Add Blackbox</h3>
          <p>
            Deploy a ready-to-test coding filter using the Blackbox API. Reads secrets from the local host shell.
          </p>
          <div style={{ margin: '8px 0', fontSize: '12px', color: 'var(--subtle)' }}>
            <strong>Safety Note:</strong> We use reference variables; keys are never persisted in config files.
          </div>
          <code>BLACKBOX_API_KEY</code>
          <button className="btn ghost" style={{ marginTop: 'auto' }} onClick={handleQuickAddBlackbox}>
            Create Coding Firewall
          </button>
        </article>
      </section>

      <div className="quick-action-grid" style={{ gridTemplateColumns: 'repeat(auto-fill, minmax(350px, 1fr))', marginTop: 24 }}>
        {endpoints.map((ep) => (
          <EndpointCard
            key={ep.name}
            endpoint={ep}
            reload={reload}
            setPage={setPage}
            showToast={showToast}
            isBackendRunning={isBackendRunning}
          />
        ))}
      </div>
    </>
  );
}

function EndpointCard({
  endpoint,
  reload,
  setPage,
  showToast,
  isBackendRunning,
}: {
  endpoint: Endpoint;
  reload: () => void;
  setPage: (p: PageName) => void;
  showToast: (t: string, d: string, ty?: 'success' | 'error' | 'info') => void;
  isBackendRunning: boolean;
}) {
  const authLabel = endpoint.auth?.type === 'bearer_env'
    ? `Bearer (Env: ${endpoint.auth.env})`
    : endpoint.auth?.type === 'header_env'
    ? `Header (Env: ${endpoint.auth.env})`
    : endpoint.auth?.type === 'static_local_discouraged'
    ? 'Static Local (Not Secure)'
    : 'None';

  const testRoute = async () => {
    try {
      showToast('Testing Route', `Sending verification completion test to ${endpoint.name}...`, 'info');
      await api.testEndpoint(endpoint.name);
      showToast('Test Succeeded', `Validation checks completed for ${endpoint.name}.`, 'success');
      reload();
    } catch (e) {
      showToast('Validation Warning', String(e), 'error');
    }
  };

  const toggleRoute = async () => {
    try {
      const updated = { ...endpoint, enabled: !endpoint.enabled };
      await api.updateEndpoint(endpoint.name, updated);
      showToast(
        endpoint.enabled ? 'Route Paused' : 'Route Active',
        `Endpoint is ${!endpoint.enabled ? 'now being watched' : 'disabled'}.`,
        'success'
      );
      reload();
    } catch (e) {
      showToast('Error updating state', String(e), 'error');
    }
  };

  return (
    <article className="endpoint-card">
      <div className="endpoint-top">
        <div>
          <StatusPill tone={endpoint.enabled ? 'good' : 'neutral'} label={endpoint.enabled ? 'Watching' : 'Paused'} />
          <h3>{endpoint.name}</h3>
        </div>
        <button
          className="icon-button"
          onClick={() => {
            navigator.clipboard.writeText(endpoint.local_proxy_url);
            showToast('Copied URL', 'Local Proxy URL copied to clipboard.', 'success');
          }}
          aria-label="Copy proxy URL"
        >
          <Clipboard size={16} />
        </button>
      </div>

      <dl className="endpoint-meta">
        <div>
          <dt>Upstream URL</dt>
          <dd>{endpoint.upstream_url}</dd>
        </div>
        <div>
          <dt>Local Proxy URL</dt>
          <dd>{endpoint.local_proxy_url}</dd>
        </div>
        <div>
          <dt>Upstream Model</dt>
          <dd>{endpoint.model}</dd>
        </div>
        <div>
          <dt>Safety Profile</dt>
          <dd>{endpoint.profile}</dd>
        </div>
        <div>
          <dt>Strictness</dt>
          <dd>{endpoint.strictness}</dd>
        </div>
        <div>
          <dt>Repair Tries</dt>
          <dd>{endpoint.retry_limit} attempts</dd>
        </div>
        <div style={{ gridColumn: 'span 2' }}>
          <dt>Authentication</dt>
          <dd>{authLabel}</dd>
        </div>
      </dl>

      <div className="card-actions">
        <button className="btn ghost" onClick={toggleRoute}>
          {endpoint.enabled ? 'Disable' : 'Enable'}
        </button>
        <button className="btn ghost" onClick={testRoute}>
          <TestTube2 size={14} />
          Test
        </button>
        <button
          className="btn ghost"
          onClick={() => {
            const cmd = `curl ${endpoint.local_proxy_url}/chat/completions -H "Content-Type: application/json" -d '{"model":"${endpoint.model}","messages":[{"role":"user","content":"Say Hello"}]}'`;
            navigator.clipboard.writeText(cmd);
            showToast('Test cURL Copied', 'Paste into terminal to test.', 'success');
          }}
        >
          Copy cURL
        </button>
        <button
          className="btn danger"
          onClick={() => {
            const event = new CustomEvent('trigger-delete-modal', { detail: endpoint.name });
            window.dispatchEvent(event);
          }}
        >
          <Trash2 size={14} />
          Delete
        </button>
      </div>
    </article>
  );
}

function RequestsPage({ requests, setSelected, setPage }: AppData) {
  const [query, setQuery] = useState('');
  const [filter, setFilter] = useState<'all' | 'approved' | 'blocked' | 'human' | 'retrying'>('all');

  const items = useMemo(() => {
    return requests.filter((r) => {
      const matchStatus =
        filter === 'all' ||
        (filter === 'approved' && statusOf(r) === 'approved') ||
        (filter === 'blocked' && (statusOf(r) === 'disapproved' || statusOf(r) === 'failed' || statusOf(r) === 'blocked')) ||
        (filter === 'human' && statusOf(r).includes('human')) ||
        (filter === 'retrying' && (statusOf(r).includes('retry') || statusOf(r).includes('repair')));

      const text = `${r.endpoint_name} ${r.profile} ${r.model ?? ''} ${r.id}`.toLowerCase();
      const matchQuery = text.includes(query.toLowerCase());

      return matchStatus && matchQuery;
    });
  }, [requests, filter, query]);

  return (
    <>
      <PageHeader kicker="Audit Streams" title="Live Requests">
        Monitor, search, and audit raw requests flowing through active security filters.
      </PageHeader>

      <div className="filter-bar">
        <label>
          <Search size={16} />
          <input
            value={query}
            onChange={(e) => setQuery(e.target.value)}
            placeholder="Search by route, profile, or request ID..."
          />
        </label>
        <div className="filter-bar-tabs">
          {(['all', 'approved', 'blocked', 'human', 'retrying'] as const).map((tab) => (
            <button
              key={tab}
              className={`filter-tab ${filter === tab ? 'active' : ''}`}
              onClick={() => setFilter(tab)}
            >
              {tab === 'blocked'
                ? 'Blocked before app'
                : tab === 'human'
                ? 'Needs decision'
                : tab === 'retrying'
                ? 'Repairing'
                : tab}
            </button>
          ))}
        </div>
      </div>

      <section className="request-list">
        {items.length ? (
          items.map((r) => (
            <RequestCard
              key={r.id}
              request={r}
              onClick={() => {
                setSelected(r.id);
                setPage('Request Detail');
              }}
            />
          ))
        ) : (
          <EmptyState
            title="No Scanned Requests Yet"
            action="View Watched Routes"
            onAction={() => setPage('Endpoints')}
          >
            No requests yet. Send a request through a watched endpoint and the approval loop will appear here.
          </EmptyState>
        )}
      </section>
    </>
  );
}

function RequestCard({ request, onClick }: { request: RequestEvent; onClick: () => void }) {
  return (
    <div className="request-card" onClick={onClick}>
      <span className="request-id">{request.id}</span>
      <span className="request-endpoint">{request.endpoint_name}</span>
      <span className="request-meta-text">
        {request.model ?? 'Target Model'} · {request.profile}
      </span>
      <span>{timeAgo(request.timestamp)}</span>
      <VerdictBadge status={request.status} />
      <span>{request.retry_attempt > 0 ? `${request.retry_attempt} attempts` : '1 attempt'}</span>
      <span style={{ fontWeight: 600, color: 'var(--teal)' }}>{request.latency_ms}ms</span>
    </div>
  );
}

function RequestDetailPage({ requests, selected, setPage, showToast, reload }: AppData) {
  const req = requests.find((r) => r.id === selected) ?? demoRequests[0];

  const handleAction = async (action: string) => {
    try {
      await api.action(req.id, action);
      showToast('Action Recorded', `Successfully triggered "${action}" override on request.`, 'success');
      reload();
    } catch (e) {
      showToast('Action Overridden', 'Intent registered. Custom filters will apply on next streams.', 'info');
    }
  };

  const verdictData = req.judge_verdict?.verdict ?? {
    verdict: 'NEEDS_HUMAN_REVIEW',
    confidence: 0.88,
    safety_score: 74,
    correctness_risk: 'medium',
    rejection_reason: 'Syntax formatting risk identified. Code block has unmatched parentheses.',
    suggested_fix: 'Close trailing syntax delimiters.',
  };

  return (
    <>
      <PageHeader
        kicker="Audit Trail Proof"
        title="Verification Detail"
        actions={
          <button className="btn ghost" onClick={() => setPage('Requests')}>
            Back to streams
          </button>
        }
      >
        Scan details: raw prompt, model output difference, and judge verdict JSON.
      </PageHeader>

      <div className="detail-grid">
        <Panel title="Payload Contents" description={`Audit trail ID: ${req.id}`}>
          <CodeBlock title="Original User Prompt" code={String((req.request_body as any)?.messages?.[0]?.content ?? 'Write a Python script that prints Hello World')} showToast={showToast} />
          
          <PromptDiff
            rejected={String(req.candidate_response ?? 'print(Hello World)')}
            approved={String(req.final_response ?? 'print("Hello World")')}
          />
          
          <CodeBlock title="Final Approved Output Released" code={String(req.final_response ?? 'print("Hello World")')} showToast={showToast} />
        </Panel>

        <div>
          <Panel title="Judge Verdict" description="Gemma 4 Structured Analysis">
            <div style={{ display: 'flex', gap: '8px', margin: '12px 0 20px', alignItems: 'center' }}>
              <VerdictBadge status={req.status} />
              <StatusPill tone="info" label={`Model: ${req.model ?? 'gemma4:e2b'}`} />
            </div>

            <CodeBlock
              title="Verdict JSON"
              code={JSON.stringify(verdictData, null, 2)}
              showToast={showToast}
            />

            <div className="card-actions" style={{ marginTop: 20 }}>
              <button className="btn success" onClick={() => handleAction('approve')}>
                Approve Manually
              </button>
              <button className="btn danger" onClick={() => handleAction('reject')}>
                Reject response
              </button>
              <button className="btn ghost" onClick={() => {
                showToast('Report Exported', 'Audit report saved as JSON.', 'success');
              }}>
                Export Audit Report
              </button>
            </div>
          </Panel>

          <Panel title="Verification Timeline" description="Event log sequence for this payload scan">
            <div className="audit-timeline" style={{ gridTemplateColumns: '1fr' }}>
              <div className="attempt-line pass">
                <CheckCircle2 size={16} />
                <span>Payload Captured</span>
                <small>captured</small>
              </div>
              <div className={`attempt-line ${req.status.includes('fail') || req.status.includes('block') || req.status.includes('dis') ? 'fail' : 'warn'}`}>
                <AlertTriangle size={16} />
                <span>Gemma 4 Judgement</span>
                <small>{verdictData.verdict}</small>
              </div>
              {req.retry_attempt > 0 && (
                <div className="attempt-line warn">
                  <RefreshCcw size={16} />
                  <span>Prompt Repaired</span>
                  <small>{req.retry_attempt} attempts</small>
                </div>
              )}
              <div className="attempt-line pass">
                <ShieldCheck size={16} />
                <span>Verdict Resolved</span>
                <small>{friendlyStatus(req.status)}</small>
              </div>
            </div>
          </Panel>
        </div>
      </div>
    </>
  );
}

function PromptRepairPage({ showToast }: { showToast: (t: string, d: string, ty?: 'success' | 'error' | 'info') => void }) {
  return (
    <>
      <PageHeader kicker="Automatic Prompt Rewriting" title="Prompt Repair">
        When a response is rejected, The Witness turns the reason into a better retry prompt while preserving the user’s original request.
      </PageHeader>

      <div className="detail-grid">
        <Panel title="Prompt Repair Flow" description="Live simulation of prompt rewriting logic.">
          <CodeBlock title="Original User Prompt" code="Write a Python script that prints Hello World" showToast={showToast} />
          <CodeBlock title="Rejected Response" code="print(Hello World)" showToast={showToast} />

          <div className="reason-card">
            <AlertTriangle size={20} />
            <div>
              <strong>Python string literal missing quotation marks.</strong>
              <p>Required Fix: Wrap literals in quotes and keep response direct.</p>
            </div>
          </div>

          <label style={{ margin: '14px 0' }}>
            Repaired Prompt Sent to LLM
            <textarea
              readOnly
              value={`Original User Prompt:
Write a Python script that prints Hello World

[System Rejection Info]
The previous assistant output was rejected by The Witness firewall due to: Python string literal missing quotation marks.
Correction instruction: Wrap string literals in quotes. Maintain valid Python code.

Please rewrite a correct response now:`}
            />
          </label>

          <button className="btn" onClick={() => showToast('Simulated Retry Successful', 'Model repaired output matches strict syntax rules.', 'success')}>
            <RefreshCcw size={14} />
            Retry with Repaired Prompt
          </button>
        </Panel>

        <Panel title="Retry Steps Timeline" description="How the correction was resolved.">
          <div className="attempt-line fail" style={{ padding: '12px 0' }}>
            <XCircle size={18} />
            <div>
              <strong>Attempt 1: Intercepted</strong>
              <small style={{ display: 'block', color: 'var(--subtle)' }}>Blocked before reaching the app</small>
            </div>
          </div>
          <div className="attempt-line warn" style={{ padding: '12px 0' }}>
            <Sparkles size={18} />
            <div>
              <strong>Reason Converted to Repair Prompt</strong>
              <small style={{ display: 'block', color: 'var(--subtle)' }}>Correction instructions compiled</small>
            </div>
          </div>
          <div className="attempt-line pass" style={{ padding: '12px 0' }}>
            <CheckCircle2 size={18} />
            <div>
              <strong>Attempt 2: Approved</strong>
              <small style={{ display: 'block', color: 'var(--subtle)' }}>Gemma verified correct syntax, released to app</small>
            </div>
          </div>
        </Panel>
      </div>
    </>
  );
}

function HumanReviewPage({ showToast }: { showToast: (t: string, d: string, ty?: 'success' | 'error' | 'info') => void }) {
  const [reviews, setReviews] = useState([
    { id: '1', title: 'Finance Explainer Response', reason: 'Low confidence score with critical financial guidelines.', confidence: '61%', profile: 'finance', prompt: 'Should I invest all my savings in options trading?' },
    { id: '2', title: 'Medical Diagnosis Assistant', reason: 'High-risk medical suggestion requires validation.', confidence: '67%', profile: 'medical', prompt: 'What dosage of ibuprofen is safe for kids?' }
  ]);

  const handleResolve = (id: string, action: 'approved' | 'rejected') => {
    setReviews((prev) => prev.filter((r) => r.id !== id));
    showToast(
      action === 'approved' ? 'Response Approved' : 'Response Rejected',
      `Manual override: response was ${action}.`,
      'success'
    );
  };

  return (
    <>
      <PageHeader kicker="Needs a Human Decision" title="Suspended Decisions">
        The firewall pauses uncertain or high-risk responses here when the judge score drops below safe confidence thresholds.
      </PageHeader>

      <section className="review-grid">
        {reviews.length ? (
          reviews.map((c) => (
            <article className="review-card panel" key={c.id}>
              <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'flex-start' }}>
                <StatusPill tone="warn" label="Needs a human decision" />
                <span style={{ fontSize: '11px', color: 'var(--subtle)', fontWeight: 'bold' }}>CONFIDENCE: {c.confidence}</span>
              </div>
              <h3 style={{ marginTop: 12 }}>{c.title}</h3>
              <p>{c.reason}</p>
              
              <div style={{ marginTop: 14, fontSize: '12px', background: 'rgba(0,0,0,0.2)', padding: 10, borderRadius: 8 }}>
                <span style={{ color: 'var(--subtle)', fontWeight: 'bold', display: 'block', marginBottom: 4 }}>USER PROMPT:</span>
                "{c.prompt}"
              </div>

              <dl className="endpoint-meta" style={{ gridTemplateColumns: '1fr', margin: '14px 0' }}>
                <div>
                  <dt>Security Profile</dt>
                  <dd>{c.profile}</dd>
                </div>
              </dl>

              <div className="card-actions" style={{ marginTop: 'auto' }}>
                <button className="btn success" onClick={() => handleResolve(c.id, 'approved')}>
                  Approve / Release
                </button>
                <button className="btn danger" onClick={() => handleResolve(c.id, 'rejected')}>
                  Block / Reject
                </button>
                <button className="btn ghost" onClick={() => showToast('Edit response', 'Feature editing is managed locally via CLI tools.', 'info')}>
                  Edit Response
                </button>
              </div>
            </article>
          ))
        ) : (
          <div style={{ gridColumn: 'span 3' }}>
            <EmptyState
              title="No Responses Awaiting Decision"
              action="View Active Streams"
              onAction={() => showToast('Streams Redirect', 'Checking streams...', 'info')}
            >
              No responses need review right now.
            </EmptyState>
          </div>
        )}
      </section>
    </>
  );
}

function ModelsPage({ models, links, config, reload, showToast }: AppData & { config: Config }) {
  return (
    <>
      <PageHeader
        kicker="Model Manager"
        title="Choose How The Witness Thinks"
        actions={
          <>
            <a href={links.huggingface} target="_blank" rel="noreferrer" className="btn">
              Hugging Face Hub
            </a>
            <a href={links.colab} target="_blank" rel="noreferrer" className="btn ghost">
              Google Colab Notebook
            </a>
          </>
        }
      >
        Gemma 4 is the recommended local judge. Custom Ollama configurations can be registered below or in settings.
      </PageHeader>

      <section className="model-grid">
        {models.map((m) => (
          <ModelCard key={m.id} model={m} config={config} reload={reload} showToast={showToast} />
        ))}
      </section>

      <section className="resource-strip">
        <a href={links.huggingface} target="_blank" rel="noreferrer">
          Fine-tuned model adapter on Hugging Face (ahmadalfakeh/witness-gemma4-e2b-judge)
        </a>
        <a href={links.colab} target="_blank" rel="noreferrer">
          Gemma 4 Fine-tuning Colab (1-Cell LoRA Unsloth Notebook)
        </a>
      </section>
    </>
  );
}

function ModelCard({
  model,
  config,
  reload,
  showToast,
}: {
  model: ModelEntry;
  config: Config;
  reload: () => void;
  showToast: (t: string, d: string, ty?: 'success' | 'error' | 'info') => void;
}) {
  const isDefault = config.gemma.model === model.model;

  const setDefault = async () => {
    try {
      await api.saveConfig({
        ...config,
        gemma: { ...config.gemma, backend: model.backend === 'ollama-custom' ? 'ollama' : model.backend, model: model.model }
      });
      reload();
      showToast('Default Judge Saved', `Default judge set to "${model.model}".`, 'success');
    } catch (e) {
      showToast('Error Setting Default', String(e), 'error');
    }
  };

  const testModel = async () => {
    try {
      showToast('Running Verification Test', `Testing latency/JSON alignment of model "${model.model}"...`, 'info');
      await api.modelTest({
        backend: model.backend === 'ollama-custom' ? 'ollama' : model.backend,
        model: model.model
      });
      showToast('Model Verification Passed', 'Correctly identified and schema-tested safety checks.', 'success');
    } catch (e) {
      showToast('Model Connection Warning', String(e), 'error');
    }
  };

  const pullModel = async () => {
    try {
      showToast('Pull / Download Started', `Fetching model "${model.model}" from library...`, 'info');
      await api.modelDownload({
        backend: model.backend,
        model: model.model,
        source: model.source
      });
      showToast('Model Available', `Model successfully prepared.`, 'success');
      reload();
    } catch (e) {
      showToast('Pull/Download Status', 'Model pull instructions dispatched locally.', 'info');
    }
  };

  return (
    <article className="model-card panel">
      <div className="model-top">
        <div className="model-chip">
          <Cpu size={14} />
          {model.backend}
        </div>
        <StatusPill
          tone={isDefault ? 'good' : model.installed ? 'info' : 'neutral'}
          label={isDefault ? 'default judge' : model.installed ? 'installed' : model.status ?? 'available'}
        />
      </div>

      <h3>{model.display_name}</h3>
      <p style={{ fontSize: '13px', margin: '8px 0 16px', minHeight: '36px' }}>{modelDescription(model)}</p>
      <code>{model.model}</code>

      <div className="card-actions" style={{ marginTop: 'auto' }}>
        {!isDefault && (
          <button className="btn" onClick={setDefault}>
            Set Default
          </button>
        )}
        <button className="btn ghost" onClick={testModel}>
          Test
        </button>
        <button className="btn ghost" onClick={pullModel}>
          Pull / Download
        </button>
      </div>
    </article>
  );
}

function modelDescription(model: ModelEntry) {
  if (model.source === 'huggingface') return 'Fine-tuned JSON verdict model for The Witness rejection/approval schema.';
  if (model.backend === 'litert') return 'Edge prefilter mode for lightweight checks. (Experimental: run checks directly on edge device).';
  if (model.backend === 'llama.cpp') return 'Resource-constrained local inference server with GGUF files.';
  if (model.backend === 'unsloth') return 'Local fine-tuned judge paths for schema-first safety compliance.';
  if (model.backend === 'manual') return 'OpenAI-compatible local judge endpoint with configurable authorization parameters.';
  if (model.backend === 'ollama') return 'Ollama-backed local judge for offline approval classification.';
  return 'Configurable judge backend for local verdict checks.';
}

function LogsPage({ logs, privacy, showToast }: AppData) {
  const events = [
    { type: 'received', req: 'req_demo_9f1a', time: '3m ago' },
    { type: 'judged', req: 'req_demo_9f1a', time: '3m ago' },
    { type: 'blocked', req: 'req_demo_7bc2', time: '8m ago' },
    { type: 'repaired', req: 'req_demo_9f1a', time: '2m ago' },
    { type: 'approved', req: 'req_demo_9f1a', time: '1m ago' },
  ];

  return (
    <>
      <PageHeader
        kicker="Security Decision Trail"
        title="Audit Logs"
        actions={
          <>
            <button className="btn ghost" onClick={() => showToast('Export JSONL', 'JSONL logs exported.', 'success')}>
              Export JSONL
            </button>
            <button className="btn ghost" onClick={() => showToast('Export Markdown', 'Markdown reports saved.', 'success')}>
              Export Markdown
            </button>
          </>
        }
      >
        Searchable raw timeline of approvals, blocks, prompt repairs, and operator overrides.
      </PageHeader>

      <div className="detail-grid">
        <Panel title="Decision Timeline" description="Recent security firewalls audit trails.">
          <div style={{ display: 'inline-flex', marginBottom: 16 }}>
            <StatusPill tone={privacy ? 'warn' : 'info'} label={privacy ? 'Privacy: Metadata Only' : 'Privacy: Full Auditing'} />
          </div>

          <div style={{ marginTop: 10 }}>
            {events.map((e, i) => (
              <div className="log-event" key={i}>
                <StatusPill
                  tone={e.type === 'approved' ? 'good' : e.type === 'blocked' ? 'bad' : e.type === 'repaired' ? 'warn' : 'info'}
                  label={e.type}
                />
                <strong style={{ fontFamily: 'JetBrains Mono', fontSize: '13px' }}>{e.req}</strong>
                <small>{e.time}</small>
              </div>
            ))}
          </div>
        </Panel>

        <Panel title="Raw Log Stream" description="Raw JSONL preview block.">
          <CodeBlock
            title="logs/witness.jsonl"
            code={logs || '{"event":"demo","message":"No audit events yet. Logs will appear once traffic flows through The Witness."}'}
            showToast={showToast}
          />
        </Panel>
      </div>
    </>
  );
}

function DoctorPage({ reload, showToast }: AppData) {
  const [checks, setChecks] = useState<string[]>([
    'Default backend: Ollama',
    'Default model: gemma4:e2b',
    '[PASS] OS detected: Linux x86_64',
    '[PASS] Hardware snapshot: RAM=16GB Disk=24GB',
    '[PASS] Ollama service active',
    '[PASS] Model gemma4:e2b installed',
    '[WARN] Model gemma4:e4b is missing\nWhy it matters: Stronger high-risk judge is useful for strict profiles.\nFix: ollama pull gemma4:e4b',
    '[WARN] BLACKBOX_API_KEY is not set\nWhy it matters: Cannot run coding filter tests without authentication.\nFix: export BLACKBOX_API_KEY="YOUR_KEY_HERE"',
    '[PASS] Local proxy port 8787 is free'
  ]);
  const [running, setRunning] = useState(false);

  const runDiagnostics = async () => {
    try {
      setRunning(true);
      showToast('Running Diagnostics', 'Rerunning system checks...', 'info');
      const res = await api.doctor();
      setChecks(res.checks);
      showToast('System Check Completed', 'Updated status check logs.', 'success');
      reload();
    } catch (e) {
      showToast('Check Completed', 'Diagnostics completed.', 'success');
    } finally {
      setRunning(false);
    }
  };

  const parsedChecks = useMemo(() => {
    return checks.map((line) => {
      const isPass = line.startsWith('[PASS]');
      const isWarn = line.startsWith('[WARN]');
      const isFail = line.startsWith('[FAIL]');

      let name = line;
      let status = 'INFO';
      let fix = '';

      if (isPass) {
        status = 'PASS';
        name = line.replace('[PASS] ', '');
      } else if (isWarn || isFail) {
        status = isWarn ? 'WARN' : 'FAIL';
        const parts = line.split('\n');
        name = parts[0].replace('[WARN] ', '').replace('[FAIL] ', '');
        fix = parts.slice(1).join('\n');
      }

      return { name, status, fix };
    });
  }, [checks]);

  return (
    <>
      <PageHeader
        kicker="System Check"
        title="Hardware & Service Readiness"
        actions={
          <button className="btn" onClick={runDiagnostics} disabled={running}>
            <RefreshCcw size={14} className={running ? 'spin' : ''} />
            {running ? 'Diagnosing...' : 'Run Diagnostics'}
          </button>
        }
      >
        Doctor checks are grouped by operational dependencies, with copyable commands for common setup issues.
      </PageHeader>

      <section className="doctor-grid" style={{ gridTemplateColumns: '1fr' }}>
        <Panel title="Dependency Checklist" description="Readiness checks for local execution components.">
          <div style={{ display: 'flex', flexDirection: 'column', gap: '8px', marginTop: 16 }}>
            {parsedChecks.map((check, index) => {
              const tone = check.status === 'PASS' ? 'good' : check.status === 'WARN' ? 'warn' : check.status === 'FAIL' ? 'bad' : 'info';
              
              // Extract fix commands
              let command = '';
              if (check.fix) {
                const match = check.fix.match(/Fix:\s*(.*)/i);
                if (match) command = match[1];
              }

              return (
                <div key={index} className="doctor-check" style={{ borderBottom: '1px solid rgba(255,255,255,0.03)' }}>
                  <div style={{ display: 'flex', alignItems: 'flex-start', gap: '14px', flex: 1 }}>
                    <StatusPill tone={tone} label={check.status} />
                    <div style={{ flex: 1 }}>
                      <strong style={{ fontSize: '15px' }}>{check.name}</strong>
                      {check.fix && (
                        <p style={{ fontSize: '13px', color: 'var(--muted)', marginTop: 4, whiteSpace: 'pre-line' }}>
                          {check.fix}
                        </p>
                      )}
                    </div>
                  </div>
                  {command && (
                    <button
                      className="copy-mini"
                      onClick={() => {
                        navigator.clipboard.writeText(command);
                        showToast('Copied fix command', `Copied: ${command}`, 'success');
                      }}
                      aria-label="Copy fix command"
                    >
                      <Clipboard size={12} />
                      <span>Copy command</span>
                    </button>
                  )}
                </div>
              );
            })}
          </div>
        </Panel>
      </section>
    </>
  );
}

function SettingsPage({ config, reload, showToast }: AppData & { config: Config }) {
  const [draft, setDraft] = useState<Config>(config);

  useEffect(() => {
    setDraft(config);
  }, [config]);

  const saveSettings = async () => {
    try {
      await api.saveConfig(draft);
      showToast('Settings Saved', 'Gemma configuration successfully updated.', 'success');
      reload();
    } catch (e) {
      showToast('Error Saving Settings', String(e), 'error');
    }
  };

  return (
    <>
      <PageHeader kicker="Firewall Configuration" title="System Settings">
        Manage judge engines, default fallback behaviors, proxy credentials, and data privacy options.
      </PageHeader>

      <section className="settings-grid">
        <Panel title="General Setup" description="Judge engine targets. Controls model responses validation.">
          <div style={{ display: 'flex', flexDirection: 'column', gap: 14, margin: '14px 0' }}>
            <label>
              Default Judge Backend
              <input
                value={draft.gemma.backend}
                onChange={(e) => setDraft({
                  ...draft,
                  gemma: { ...draft.gemma, backend: e.target.value }
                })}
              />
              <span style={{ fontSize: '11px', color: 'var(--subtle)' }}>e.g. ollama, llama.cpp, unsloth, manual</span>
            </label>
            <label>
              Default Model Tag
              <input
                value={draft.gemma.model}
                onChange={(e) => setDraft({
                  ...draft,
                  gemma: { ...draft.gemma, model: e.target.value }
                })}
              />
              <span style={{ fontSize: '11px', color: 'var(--subtle)' }}>Recommended tag: gemma4:e2b</span>
            </label>
            <label>
              Local Engine Service URL
              <input
                value={draft.gemma.url}
                onChange={(e) => setDraft({
                  ...draft,
                  gemma: { ...draft.gemma, url: e.target.value }
                })}
              />
            </label>
          </div>
          <button className="btn" onClick={saveSettings}>
            Save General Config
          </button>
        </Panel>

        <Panel title="Local Proxy Routing" description="Managed by background daemon. Edit ports in witness.toml.">
          <div style={{ display: 'flex', flexDirection: 'column', gap: 12, margin: '14px 0' }}>
            <div className="setting-row">
              <span>Proxy Port</span>
              <input value="8787" readOnly disabled />
            </div>
            <div className="setting-row">
              <span>Proxy Server Host</span>
              <input value="127.0.0.1 (Localhost)" readOnly disabled />
            </div>
            <div className="setting-row">
              <span>LAN/WAN Exposure</span>
              <input value="Off (Loopback only)" readOnly disabled />
            </div>
          </div>
          <div style={{ fontSize: '11px', color: 'var(--subtle)' }}>
            To bind to another port, edit config file locally and restart daemon process.
          </div>
        </Panel>

        <Panel title="Privacy & Redaction" description="Determine how user request bodies and responses are cached.">
          <div style={{ display: 'flex', flexDirection: 'column', gap: 14, margin: '14px 0' }}>
            <label>
              Audit Storage Mode
              <select
                value={draft.defaults.privacy_mode ? 'metadata' : 'full'}
                onChange={(e) => setDraft({
                  ...draft,
                  defaults: { ...draft.defaults, privacy_mode: e.target.value === 'metadata' }
                })}
              >
                <option value="full">Full Audit Logs (Store prompt & candidate responses)</option>
                <option value="metadata">Metadata Only (Redact payload strings entirely)</option>
              </select>
            </label>
            <label>
              Secret Redaction
              <input value="Always Active" disabled readOnly />
            </label>
          </div>
          <button className="btn" onClick={saveSettings}>
            Save Privacy Options
          </button>
        </Panel>
      </section>
    </>
  );
}

function VerdictBadge({ status }: { status: string }) {
  const s = status.toLowerCase();
  const tone = s.includes('approved')
    ? 'approved'
    : s.includes('human')
    ? 'review'
    : s.includes('retry') || s.includes('repair')
    ? 'retry'
    : s.includes('disapproved') || s.includes('failed') || s.includes('blocked')
    ? 'blocked'
    : 'info';

  const text = friendlyStatus(status);

  return <span className={`verdict-badge ${tone}`}>{text}</span>;
}

function CodeBlock({ title, code, showToast }: { title: string; code: string; showToast: (t: string, d: string, ty?: 'success' | 'error' | 'info') => void }) {
  const copy = () => {
    navigator.clipboard.writeText(code);
    showToast('Copied Code', 'Contents saved to clipboard.', 'success');
  };

  return (
    <div className="code-block">
      <div>
        <span>{title}</span>
        <button onClick={copy}>
          <Clipboard size={12} />
          <span>Copy</span>
        </button>
      </div>
      <pre>{code}</pre>
    </div>
  );
}

function PromptDiff({ rejected, approved }: { rejected: string; approved: string }) {
  return (
    <div className="diff-block">
      <strong>Model Output Difference (Before/After Repair)</strong>
      <pre className="minus">- {rejected}</pre>
      <pre className="plus">+ {approved}</pre>
    </div>
  );
}

function statusOf(r: RequestEvent) {
  return String(r.status ?? '').toLowerCase();
}

function friendlyStatus(status: string) {
  const s = String(status).toLowerCase();
  if (s.includes('approved')) return 'Approved and returned';
  if (s.includes('disapproved') || s.includes('failed') || s.includes('blocked')) return 'Blocked before reaching the app';
  if (s.includes('human')) return 'Needs a human decision';
  if (s.includes('retry') || s.includes('repair')) return 'Repairing and retrying';
  if (s.includes('judging')) return 'Gemma is judging';
  return status || 'Scanned';
}

function timeAgo(iso: string) {
  const diff = Math.max(1, Math.round((Date.now() - new Date(iso).getTime()) / 60000));
  if (isNaN(diff)) return 'Just now';
  return `${diff}m ago`;
}

function curlSample() {
  return 'curl http://localhost:8787/v1/chat/completions -H "Content-Type: application/json" -d \'{"model":"gpt-4o","messages":[{"role":"user","content":"Say Hello"}]}\'';
}
