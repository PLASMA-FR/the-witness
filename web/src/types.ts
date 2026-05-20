export type Strictness = 'relaxed'|'medium'|'high'|'critical';
export type FallbackMode = 'human_review'|'demo_judge'|'safe_response'|'error';
export interface EndpointAuth { type: string; env?: string; value?: string }
export interface Endpoint { name:string; enabled:boolean; upstream_url:string; local_proxy_url:string; model:string; profile:string; retry_limit:number; strictness:Strictness; fallback_mode:FallbackMode; auth_header?:string|null; auth?:EndpointAuth|null; judge_backend?:string|null; judge_model?:string|null; timeout_seconds:number }
export interface Config { gemma:{backend:string; model:string; url:string; setup_completed:boolean; auth_header?:string|null}; setup:{last_doctor_check:string; judge_schema_test_passed:boolean; proxy_test_passed:boolean; model_test_passed:boolean; demo_mode:boolean}; defaults:{retry_limit:number; strictness:Strictness; fallback_mode:FallbackMode; log_format:string; privacy_mode:boolean}; endpoints: Endpoint[]; profiles: Record<string, unknown> }
export interface TailscaleDashboardAccess { detected:boolean; available:boolean; ip?:string|null; url?:string|null; hint:string }
export interface DashboardAccess { bind_url:string; local_url:string; tailscale:TailscaleDashboardAccess }
export interface Health { ok:boolean; service:string; service_running?:boolean; dashboard:string; dashboard_access?:DashboardAccess; proxy:string; setup_ready:boolean; backend:string; model:string; loopback_only:boolean }
export interface RequestEvent { id:string; endpoint_name:string; model?:string; profile:string; status:string; retry_attempt:number; latency_ms:number; timestamp:string; judge_verdict?: { verdict: { verdict: string } } | null; request_body?: unknown; candidate_response?: unknown; final_response?: unknown }
export interface ModelEntry { id:string; display_name:string; backend:string; base_model?:string; model:string; source:string; slug?:string; local_path?:string; installed?:boolean; status?:string }
