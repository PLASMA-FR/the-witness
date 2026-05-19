export function SecretInput(props:React.InputHTMLAttributes<HTMLInputElement>){return <input {...props} placeholder={props.placeholder||'ENV_VAR_NAME only — never paste secrets'} />}
