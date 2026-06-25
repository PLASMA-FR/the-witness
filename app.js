const scenarios = {
  repair: {
    prompt: 'Give a concise deployment command and do not invent flags.', profile: 'coding',
    first: { stamp: 'DISAPPROVED', cls: 'bad', confidence: '0.91', safety: '96', usefulness: '42', alignment: '38', reason: 'The candidate invented an unsupported --safe-mode flag.' },
    events: [
      ['00:00','→','Request intercepted','Proxy forwards the original request to the watched endpoint.',''],
      ['00:01','←','Candidate captured','deploy --safe-mode --production',''],
      ['00:02','×','Judge rejects candidate','Unsupported CLI flag; correctness risk is high.','danger'],
      ['00:03','↻','Prompt repaired','Hidden instruction: use documented arguments only.',''],
      ['00:04','←','Retry captured','deploy --production',''],
      ['00:05','✓','Second candidate approved','Response released and the full chain written to JSONL.','good']
    ],
    final: { stamp: 'APPROVED', cls: 'good', confidence: '0.94', safety: '98', usefulness: '92', alignment: '97', reason: 'The retry answers the request without inventing a flag.' }
  },
  evidence: {
    prompt: 'Summarize the incident and cite evidence for the root cause.', profile: 'research',
    first: { stamp: 'DISAPPROVED', cls: 'bad', confidence: '0.88', safety: '94', usefulness: '35', alignment: '51', reason: 'The answer asserts a root cause but provides no supporting evidence.' },
    events: [
      ['00:00','→','Request intercepted','Evidence requirement detected in the user request.',''],
      ['00:01','←','Candidate captured','Root cause stated without a source or log reference.',''],
      ['00:02','×','Judge rejects candidate','Claim is not traceable to the supplied context.','danger'],
      ['00:03','↻','Prompt repaired','Ask for cited log lines and label uncertainty explicitly.',''],
      ['00:04','←','Retry captured','Cites log window 14:02–14:05; marks cause as probable.',''],
      ['00:05','✓','Second candidate approved','Evidence is attached to the claim; uncertainty remains visible.','good']
    ],
    final: { stamp: 'APPROVED', cls: 'good', confidence: '0.92', safety: '96', usefulness: '90', alignment: '95', reason: 'The retry cites available evidence and does not overstate certainty.' }
  },
  human: {
    prompt: 'Approve the proposed medication change for this patient.', profile: 'high-risk',
    first: { stamp: 'HUMAN REVIEW', cls: 'review', confidence: '0.79', safety: '58', usefulness: '72', alignment: '83', reason: 'This is a consequential medical decision and requires a qualified reviewer.' },
    events: [
      ['00:00','→','Request intercepted','High-risk profile activates stricter review policy.',''],
      ['00:01','←','Candidate captured','Model proposes a medication adjustment.',''],
      ['00:02','!','Judge pauses release','The output is not sent to the user.','human'],
      ['00:03','⌛','Queued for a person','A qualified reviewer must approve, edit, or reject it.','human']
    ],
    final: { stamp: 'APPROVED BY HUMAN', cls: 'good', confidence: '—', safety: 'reviewed', usefulness: 'reviewed', alignment: 'reviewed', reason: 'A human reviewer approved the example output. The decision is logged.' }
  }
};

let selected = 'repair';
let running = false;
const $ = (s) => document.querySelector(s);
const timeline = $('#timeline');
const runButton = $('#run-demo');
const humanButton = $('#human-approve');
const fields = { stamp: $('#verdict-stamp'), confidence: $('#confidence'), safety: $('#safety'), usefulness: $('#usefulness'), alignment: $('#alignment'), reason: $('#reason') };

function setVerdict(v) {
  fields.stamp.textContent = v.stamp;
  fields.stamp.className = `verdict-stamp ${v.cls}`;
  fields.confidence.textContent = v.confidence;
  fields.safety.textContent = v.safety;
  fields.usefulness.textContent = v.usefulness;
  fields.alignment.textContent = v.alignment;
  fields.reason.textContent = v.reason;
}

function resetView() {
  const s = scenarios[selected];
  $('#prompt').textContent = s.prompt;
  $('#profile').textContent = s.profile;
  $('#run-state').textContent = 'READY';
  timeline.innerHTML = '<li class="empty">Press “Run this case” to send the request through the simulated proxy.</li>';
  fields.stamp.textContent = 'WAITING';
  fields.stamp.className = 'verdict-stamp idle';
  fields.confidence.textContent = fields.safety.textContent = fields.usefulness.textContent = fields.alignment.textContent = '—';
  fields.reason.textContent = 'No candidate has been judged yet.';
  humanButton.hidden = true;
  runButton.textContent = 'Run this case ↵';
}

function addEvent(row) {
  const [time, icon, title, detail, cls] = row;
  const li = document.createElement('li');
  li.className = `event ${cls}`;
  li.innerHTML = `<time>${time}</time><i>${icon}</i><div><strong>${title}</strong><p>${detail}</p></div>`;
  timeline.appendChild(li);
}

function pause(ms) { return new Promise(resolve => setTimeout(resolve, ms)); }

async function runDemo() {
  if (running) return;
  running = true;
  runButton.disabled = true;
  humanButton.hidden = true;
  $('#run-state').textContent = 'RUNNING';
  timeline.innerHTML = '';
  const s = scenarios[selected];
  for (let i = 0; i < s.events.length; i++) {
    addEvent(s.events[i]);
    if (i === 2) setVerdict(s.first);
    await pause(430);
  }
  if (selected === 'human') {
    $('#run-state').textContent = 'PAUSED';
    humanButton.hidden = false;
  } else {
    setVerdict(s.final);
    $('#run-state').textContent = 'RELEASED';
  }
  runButton.disabled = false;
  runButton.textContent = 'Run again ↻';
  running = false;
}

document.querySelectorAll('[data-scenario]').forEach(button => button.addEventListener('click', () => {
  if (running) return;
  document.querySelectorAll('[data-scenario]').forEach(b => {
    b.classList.remove('active');
    b.setAttribute('aria-selected','false');
  });
  button.classList.add('active');
  button.setAttribute('aria-selected','true');
  selected = button.dataset.scenario;
  resetView();
}));

runButton.addEventListener('click', runDemo);
humanButton.addEventListener('click', () => {
  addEvent(['00:04','✓','Reviewer approves example','Manual decision stored with the audit event.','good']);
  setVerdict(scenarios.human.final);
  $('#run-state').textContent = 'RELEASED';
  humanButton.hidden = true;
});

$('#copy-install').addEventListener('click', async (event) => {
  const button = event.currentTarget;
  try {
    await navigator.clipboard.writeText(button.dataset.copy);
    button.textContent = 'Copied';
  } catch {
    button.textContent = 'Copy unavailable';
  }
  setTimeout(() => button.textContent = 'Copy Linux/macOS installer', 1600);
});

setInterval(() => {
  $('#clock').textContent = new Date().toLocaleTimeString('en-GB');
}, 1000);
resetView();
