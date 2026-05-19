export function VerdictBadge({value}:{value?:string}){const v=(value||'pending').toLowerCase(); return <span className={`badge ${v}`}>{value||'pending'}</span>}
