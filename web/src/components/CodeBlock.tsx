export function CodeBlock({value}:{value:unknown}){return <pre className='mono'>{typeof value==='string'?value:JSON.stringify(value,null,2)}</pre>}
