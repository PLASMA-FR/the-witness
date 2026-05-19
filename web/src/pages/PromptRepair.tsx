const template = `Original user request:

Rejected response:

Rejection reason:

Required fix:

Now generate a corrected answer.`;
export function PromptRepair(){return <><div className='topbar'><div><h1>Prompt Repair</h1><p style={{color:'var(--muted)'}}>Edit repaired prompts, retry, regenerate, or send to human review.</p></div></div><section className='card'><textarea style={{width:'100%',minHeight:220}} defaultValue={template}/><div className='row-actions'><button className='btn primary'>Retry</button><button className='btn'>Regenerate repair</button><button className='btn'>Send to human review</button></div></section></>}
