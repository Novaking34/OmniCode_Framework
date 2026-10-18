// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( OmniCode GUI Server Identity, Zero-Dependency Architecture & REST Invariants )
// ==============================================================================
//! OmniCode Local Web GUI Server & Terminal Gateway
//!
//! Provides a pure standard-library HTTP server (0 external crates) hosting the
//! local OmniCode Studio GUI and Terminal Interface:
//!   - Static file delivery: `gui/index.html`, `gui/style.css`, `gui/app.js`
//!   - REST endpoints:
//!     * `POST /api/prompt`    -> Executes AICortex neural-symbolic cognitive pipeline
//!     * `POST /api/terminal`  -> Executes CLI commands directly (`omnicode run/lint/demo`)
//!     * `GET  /api/manifest`  -> Returns canonical `manifest.omni` covenant anchor
//!     * `GET  /api/lexicon`   -> Returns offline Biblical world model & passage citations
//!     * `GET  /api/scrolls`   -> Returns episodic memory learning statistics

use std::fs;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::thread;

use crate::ai::{AICortex, AILearner, BiblicalLexicon, BrainVisualizer};
use crate::core::ternary::QuadState;
use crate::diagnostics::hover::HoverEngine;
use crate::diagnostics::logger::DiagnosticLogger;
use crate::terminal::{create_shared_scheduler, SharedScheduler, TerminalSession};


// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( Server Configuration, HTTP Parser & Response Structures )
// ==============================================================================

pub struct GuiServerConfig {
    pub host: String,
    pub port: u16,
    pub gui_dir: String,
}

impl Default for GuiServerConfig {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 8080,
            gui_dir: "gui".to_string(),
        }
    }
}


// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( TCP Listener Loop, HTTP Routing, and REST API Handlers )
// ==============================================================================

/// Starts the OmniCode GUI HTTP server on the specified port.
pub fn run_gui_server(port: u16) -> Result<(), String> {
    let host = std::env::var("OMNICODE_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let listener = TcpListener::bind(format!("{host}:{port}"))
        .map_err(|e| format!("Failed to bind TCP listener to {host}:{port}: {e}"))?;

    println!("\x1b[1;36m===============================================================================\x1b[0m");
    println!("\x1b[1;36m  OmniCode GUI Studio & Terminal Server Active                                \x1b[0m");
    println!("\x1b[1;36m===============================================================================\x1b[0m");
    println!("\x1b[1;32m✓ Web Studio URL:\x1b[0m http://localhost:{port} (listening on {host}:{port})");
    println!("\x1b[1;33m✓ Backend Mode:\x1b[0m 100% Offline | Grounded in Biblical Lexicon & Covenant Manifest");
    println!("\x1b[1;35m✓ Terminal Gateway:\x1b[0m Active (CLI commands executable via GUI)\n");
    println!("Press Ctrl+C to terminate server.\n");

    let lexicon = Arc::new(BiblicalLexicon::new());
    let scheduler = create_shared_scheduler();
    let session = Arc::new(Mutex::new(TerminalSession::new(Arc::clone(&scheduler))));

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let lex_clone = Arc::clone(&lexicon);
                let sched_clone = Arc::clone(&scheduler);
                let sess_clone = Arc::clone(&session);
                thread::spawn(move || {
                    handle_connection(stream, lex_clone, sched_clone, sess_clone);
                });
            }
            Err(e) => {
                eprintln!("Connection failed: {e}");
            }
        }
    }

    Ok(())
}

fn handle_connection(
    mut stream: TcpStream,
    lexicon: Arc<BiblicalLexicon>,
    scheduler: SharedScheduler,
    session: Arc<Mutex<TerminalSession>>,
) {
    let mut buffer = [0u8; 16384];
    let bytes_read = match stream.read(&mut buffer) {
        Ok(n) if n > 0 => n,
        _ => return,
    };

    let request_str = String::from_utf8_lossy(&buffer[..bytes_read]);
    let mut lines = request_str.lines();
    let request_line = match lines.next() {
        Some(line) => line,
        std::option::Option::None => return,
    };

    let parts: Vec<&str> = request_line.split_whitespace().collect();
    if parts.len() < 2 {
        return;
    }

    let method = parts[0];
    let raw_path = parts[1];
    let path = raw_path.split('?').next().unwrap_or(raw_path);
    let is_head = method == "HEAD";
    let route_method = if is_head { "GET" } else { method };

    // Extract body if POST
    let body = if method == "POST" {
        if let Some(pos) = request_str.find("\r\n\r\n") {
            request_str[pos + 4..].to_string()
        } else if let Some(pos) = request_str.find("\n\n") {
            request_str[pos + 2..].to_string()
        } else {
            String::new()
        }
    } else {
        String::new()
    };

    // Route Request
    match (route_method, path) {
        ("GET", "/") | ("GET", "/studio") | ("GET", "/index.html") | ("GET", "/studio.html") => {
            serve_file(&mut stream, "apps/gui/studio.html", "text/html; charset=utf-8", is_head);
        }
        ("GET", "/terminal") | ("GET", "/terminal.html") => {
            serve_file(&mut stream, "apps/gui/terminal.html", "text/html; charset=utf-8", is_head);
        }
        ("GET", "/specs") | ("GET", "/specs.html") => {
            serve_file(&mut stream, "apps/gui/specs.html", "text/html; charset=utf-8", is_head);
        }
        ("GET", "/model") | ("GET", "/model.html") => {
            serve_file(&mut stream, "apps/gui/model.html", "text/html; charset=utf-8", is_head);
        }
        ("GET", "/lexicon") | ("GET", "/lexicon.html") => {
            serve_file(&mut stream, "apps/gui/lexicon.html", "text/html; charset=utf-8", is_head);
        }
        ("GET", "/scrolls") | ("GET", "/scrolls.html") => {
            serve_file(&mut stream, "apps/gui/scrolls.html", "text/html; charset=utf-8", is_head);
        }
        ("GET", "/cloud") | ("GET", "/cloud.html") => {
            serve_file(&mut stream, "apps/gui/cloud.html", "text/html; charset=utf-8", is_head);
        }
        ("GET", "/download") | ("GET", "/download.html") => {
            serve_file(&mut stream, "apps/gui/download.html", "text/html; charset=utf-8", is_head);
        }
        ("GET", "/terminal.js") => {
            serve_file(&mut stream, "apps/gui/terminal.js", "application/javascript; charset=utf-8", is_head);
        }
        ("GET", "/style.css") => {
            serve_file(&mut stream, "apps/gui/style.css", "text/css; charset=utf-8", is_head);
        }
        ("GET", "/app.js") => {
            serve_file(&mut stream, "apps/gui/app.js", "application/javascript; charset=utf-8", is_head);
        }
        ("GET", "/api/health") => {
            let json = r#"{"status":"healthy","engine":"OmniCode","version":"0.1.0","state":"+1:PROCEED"}"#;
            send_json_response(&mut stream, 200, json, is_head);
        }
        ("GET", "/api/download") => {
            let archive_path = "dist/omnicode-terminal-linux-x86_64.tar.gz";
            if let Ok(bytes) = fs::read(archive_path) {
                let header = format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: application/gzip\r\nContent-Disposition: attachment; filename=\"omnicode-terminal-linux-x86_64.tar.gz\"\r\nContent-Length: {}\r\nAccess-Control-Allow-Origin: *\r\nConnection: close\r\n\r\n",
                    bytes.len()
                );
                let _ = stream.write_all(header.as_bytes());
                if !is_head {
                    let _ = stream.write_all(&bytes);
                }
            } else {
                let err = r#"{"error":"Distribution archive not found. Run scripts/package_native.sh to build."}"#;
                send_json_response(&mut stream, 404, err, is_head);
            }
        }
        ("GET", "/api/scheduler") => {
            let json = scheduler.lock().unwrap().to_json();
            send_json_response(&mut stream, 200, &json, is_head);
        }
        ("POST", "/api/scheduler/spawn") => {
            let cmd_str = extract_json_field(&body, "command").unwrap_or_else(|| "omnicode demo".to_string());
            let name_str = extract_json_field(&body, "name").unwrap_or_else(|| "UserTask".to_string());
            let prio_str = extract_json_field(&body, "priority").unwrap_or_else(|| "NORMAL".to_string());
            let priority = match prio_str.to_uppercase().as_str() {
                "HIGH" => crate::terminal::TaskPriority::High,
                "LOW" => crate::terminal::TaskPriority::Low,
                "BACKGROUND" => crate::terminal::TaskPriority::Background,
                _ => crate::terminal::TaskPriority::Normal,
            };
            let mut sched = scheduler.lock().unwrap();
            let pid = sched.spawn(&name_str, &cmd_str, priority);
            let json = format!(r#"{{"pid":{},"status":"spawned"}}"#, pid);
            send_json_response(&mut stream, 200, &json, is_head);
        }
        ("GET", "/api/manifest") => {
            let manifest_content = fs::read_to_string("manifest.omni")
                .unwrap_or_else(|_| "# Manifest not found".to_string());
            let json = format!(
                r#"{{"manifest": "{}"}}"#,
                escape_json_string(&manifest_content)
            );
            send_json_response(&mut stream, 200, &json, is_head);
        }
        ("GET", "/api/lexicon") => {
            let mut entries_json = Vec::new();
            for e in lexicon.all_entries() {
                entries_json.push(format!(
                    r#"{{"term":"{}","domain":"{}","definition":"{}","kjv":"{}","web":"{}","state":"{}","guidance":"{}"}}"#,
                    escape_json_string(e.term),
                    escape_json_string(e.domain.as_str()),
                    escape_json_string(e.definition),
                    escape_json_string(e.kjv_passage),
                    escape_json_string(e.web_passage),
                    escape_json_string(e.quad_state_resonance.macro_op()),
                    escape_json_string(e.moral_guidance)
                ));
            }
            let json = format!(r#"{{"count":{},"entries":[{}]}}"#, entries_json.len(), entries_json.join(","));
            send_json_response(&mut stream, 200, &json, is_head);
        }
        ("GET", "/api/scrolls") => {
            let scroll_dir = if Path::new("storage/scrolls").exists() {
                "storage/scrolls"
            } else if Path::new("runtime/scrolls").exists() {
                "runtime/scrolls"
            } else {
                "scrolls"
            };
            let learner = AILearner::new(scroll_dir);
            let summary = learner.digest_experiences().unwrap_or_default();
            let mut exp_json = Vec::new();
            for e in &summary.experiences {
                exp_json.push(format!(
                    r#"{{"session_id":"{}","prompt":"{}","state":"{}","repairs":{},"failpoints":{},"scroll":"{}"}}"#,
                    escape_json_string(&e.session_id),
                    escape_json_string(&e.prompt),
                    escape_json_string(e.final_state.macro_op()),
                    e.repair_count,
                    e.failpoint_count,
                    escape_json_string(&e.scroll_path.to_string_lossy())
                ));
            }
            let json = format!(
                r#"{{"total":{},"proceed":{},"rollback":{},"await":{},"halt":{},"repairs_learned":{},"episodes":[{}]}}"#,
                summary.total_episodes, summary.proceed_count, summary.rollback_count, summary.await_count, summary.halt_count,
                summary.total_repairs_learned, exp_json.join(",")
            );
            send_json_response(&mut stream, 200, &json, is_head);
        }
        ("POST", "/api/prompt") => {
            let prompt = extract_json_field(&body, "prompt").unwrap_or_else(|| body.clone());
            let mut cortex = AICortex::new();
            match cortex.process_prompt(&prompt) {
                Ok(res) => {
                    let mut repairs_json = Vec::new();
                    for r in &res.repairs_applied {
                        repairs_json.push(format!(
                            r#"{{"line":{},"kind":"{:?}","desc":"{}"}}"#,
                            r.line, r.kind, escape_json_string(&r.description)
                        ));
                    }
                    let json = format!(
                        r#"{{"session_id":"{}","prompt":"{}","state":"{}","response":"{}","omnicode":"{}","scroll":"{}","log":"{}","repairs":[{}]}}"#,
                        escape_json_string(&res.session_id),
                        escape_json_string(&res.prompt),
                        escape_json_string(res.execution_state.macro_op()),
                        escape_json_string(&res.natural_response),
                        escape_json_string(&res.synthesized_omnicode),
                        escape_json_string(&res.scroll_path.to_string_lossy()),
                        escape_json_string(&res.log_path.to_string_lossy()),
                        repairs_json.join(",")
                    );
                    send_json_response(&mut stream, 200, &json, is_head);
                }
                Err(e) => {
                    let json = format!(r#"{{"error":"{}"}}"#, escape_json_string(&e));
                    send_json_response(&mut stream, 500, &json, is_head);
                }
            }
        }
        ("GET", "/api/system") => {
            let json = r#"{"project":"OmniCode","organization":"CreativeWorkzStudio LLC","version":"0.1.0","offline":true,"engine":"Rust Core 2024 (Balanced Ternary Quad-State)"}"#;
            send_json_response(&mut stream, 200, json, is_head);
        }
        ("GET", "/api/brain/map") => {
            let cortex = AICortex::new();
            let (exc, neu, inh) = cortex.brain.synaptic_counts();
            let total = exc + neu + inh;
            let crystal_ratio = if total > 0 { (exc + inh) as f64 / total as f64 } else { 0.0 };
            let ascii = BrainVisualizer::render_ascii_map(&cortex.brain, "CPI-SI Base Substrate");

            let mut layers_json = Vec::new();
            for (idx, layer) in cortex.brain.layers.iter().enumerate() {
                let n_count = layer.neurons.len();
                let w_count = if n_count > 0 { layer.neurons[0].weights.len() } else { 0 };
                layers_json.push(format!(
                    r#"{{"layer":{},"neurons":{},"weights_per_neuron":{}}}"#,
                    idx + 1, n_count, w_count
                ));
            }
            let json = format!(
                r#"{{"total_synapses":{},"excitatory":{},"neutral":{},"inhibitory":{},"crystallization_ratio":{:.4},"ascii_map":"{}","layers":[{}]}}"#,
                total, exc, neu, inh, crystal_ratio, escape_json_string(&ascii), layers_json.join(",")
            );
            send_json_response(&mut stream, 200, &json, is_head);
        }
        ("GET", "/api/brain/growth") => {
            let cortex = AICortex::new();
            let timeline_ascii = BrainVisualizer::render_growth_timeline(&cortex.growth_history);
            let mut snapshots_json = Vec::new();
            for s in &cortex.growth_history.snapshots {
                snapshots_json.push(format!(
                    r#"{{"milestone":"{}","excitatory":{},"neutral":{},"inhibitory":{},"total":{},"crystallization":{:.4},"noise_resilience":{:.4}}}"#,
                    escape_json_string(&s.epoch_or_milestone), s.excitatory_count, s.neutral_count, s.inhibitory_count, s.total_synapses, s.crystallization_ratio, s.noise_resilience
                ));
            }
            let json = format!(
                r#"{{"count":{},"timeline_ascii":"{}","snapshots":[{}]}}"#,
                cortex.growth_history.snapshots.len(), escape_json_string(&timeline_ascii), snapshots_json.join(",")
            );
            send_json_response(&mut stream, 200, &json, is_head);
        }
        ("POST", "/api/brain/train-curriculum") => {
            let weights_path = PathBuf::from("storage/weights/cpisi_base.weights");
            match crate::cli::commands::run_curriculum_training(&weights_path, true) {
                Ok(report_str) => {
                    let json = format!(r#"{{"status":"ok","report":"{}"}}"#, escape_json_string(&report_str));
                    send_json_response(&mut stream, 200, &json, is_head);
                }
                Err(e) => {
                    let json = format!(r#"{{"error":"{}"}}"#, escape_json_string(&e));
                    send_json_response(&mut stream, 500, &json, is_head);
                }
            }
        }
        ("GET", "/api/curriculum") => {
            let curriculum = crate::ai::Curriculum::builtin_25_chapters();
            let mut ch_json = Vec::new();
            for ch in &curriculum.chapters {
                ch_json.push(format!(
                    r#"{{"volume":"{}","chapter_id":"{}","title":"{}","prompt":"{}","files":{},"lines":{},"words":{},"target_state":"{}"}}"#,
                    escape_json_string(&ch.volume_id),
                    escape_json_string(&ch.chapter_id),
                    escape_json_string(&ch.title),
                    escape_json_string(&ch.prompt),
                    ch.file_count,
                    ch.line_count,
                    ch.word_count,
                    escape_json_string(ch.target_state.macro_op())
                ));
            }
            let json = format!(r#"{{"count":{},"chapters":[{}]}}"#, ch_json.len(), ch_json.join(","));
            send_json_response(&mut stream, 200, &json, is_head);
        }
        ("GET", "/api/ai/tools") => {
            let tools_text = crate::cli::commands::render_ai_tools_overview();
            let json = format!(
                r#"{{"tools":["LoggerTool","DebuggerTool","LinterTool","RestoratorTool","HoverTool"],"overview":"{}"}}"#,
                escape_json_string(&tools_text)
            );
            send_json_response(&mut stream, 200, &json, is_head);
        }
        ("POST", "/api/ai/heal") => {
            let code = extract_json_field(&body, "code");
            let file = extract_json_field(&body, "file");
            let tool_belt = crate::ai::AIToolBelt::default();
            if let Some(f) = file {
                match tool_belt.restorator.heal_file(&f, false) {
                    Ok(rep) => {
                        let agreement = tool_belt.restorator.check_two_way_agreement(&tool_belt.debugger.debugger, &rep.restored_content);
                        let json = format!(
                            r#"{{"file":"{}","was_modified":{},"restored":"{}","verified":{},"two_way_checksum":{}}}"#,
                            escape_json_string(&f), rep.was_modified, escape_json_string(&rep.restored_content), rep.verified, agreement.is_certified()
                        );
                        send_json_response(&mut stream, 200, &json, is_head);
                    }
                    Err(e) => {
                        let json = format!(r#"{{"error":"{}"}}"#, escape_json_string(&e));
                        send_json_response(&mut stream, 500, &json, is_head);
                    }
                }
            } else if let Some(c) = code {
                let rep = tool_belt.restorator.heal_source(&c);
                let agreement = tool_belt.restorator.check_two_way_agreement(&tool_belt.debugger.debugger, &rep.restored_content);
                let json = format!(
                    r#"{{"was_modified":{},"restored":"{}","verified":{},"two_way_checksum":{}}}"#,
                    rep.was_modified, escape_json_string(&rep.restored_content), rep.verified, agreement.is_certified()
                );
                send_json_response(&mut stream, 200, &json, is_head);
            } else {
                let json = r#"{"error":"Missing 'code' or 'file' parameter"}"#;
                send_json_response(&mut stream, 400, json, is_head);
            }
        }
        ("GET", "/api/hover") => {
            let file = extract_query_param(raw_path, "file");
            let symbol = extract_query_param(raw_path, "symbol");
            let line = extract_query_param(raw_path, "line")
                .and_then(|s| s.parse::<usize>().ok())
                .unwrap_or(1);
            let col = extract_query_param(raw_path, "col")
                .and_then(|s| s.parse::<usize>().ok())
                .unwrap_or(1);

            let mut engine = HoverEngine::new(DiagnosticLogger::new(false));
            engine.index_workspace(".");

            let card = if let Some(ref sym) = symbol {
                engine.document_symbol(sym, file.as_deref())
            } else if let Some(ref f) = file {
                engine.hover_at(f, line, col)
            } else {
                None
            };

            match card {
                Some(c) => send_json_response(&mut stream, 200, &c.to_json(), is_head),
                None => {
                    let err = r#"{"symbol":"","found":false,"error":"No symbol found or target could not be resolved"}"#;
                    send_json_response(&mut stream, 404, err, is_head);
                }
            }
        }
        ("POST", "/api/hover") => {
            let file = extract_json_field(&body, "file");
            let symbol = extract_json_field(&body, "symbol");
            let line = extract_json_field(&body, "line")
                .and_then(|s| s.parse::<usize>().ok())
                .unwrap_or(1);
            let col = extract_json_field(&body, "col")
                .and_then(|s| s.parse::<usize>().ok())
                .unwrap_or(1);

            let mut engine = HoverEngine::new(DiagnosticLogger::new(false));
            engine.index_workspace(".");

            let card = if let Some(ref sym) = symbol {
                engine.document_symbol(sym, file.as_deref())
            } else if let Some(ref f) = file {
                engine.hover_at(f, line, col)
            } else {
                None
            };

            match card {
                Some(c) => send_json_response(&mut stream, 200, &c.to_json(), is_head),
                None => {
                    let err = r#"{"symbol":"","found":false,"error":"No symbol found or target could not be resolved"}"#;
                    send_json_response(&mut stream, 404, err, is_head);
                }
            }
        }
        ("POST", "/api/terminal") => {
            let cmd_str = extract_json_field(&body, "command").unwrap_or_else(|| body.clone());
            let clean_cmd = cmd_str.trim();

            let res = {
                let mut s = session.lock().unwrap();
                s.execute(clean_cmd)
            };

            let state_str = match res.state {
                QuadState::Proceed => "+1:PROCEED",
                QuadState::Await => "0:AWAIT",
                QuadState::Rollback => "-1:ROLLBACK",
                QuadState::Halt => "NULL:HALT",
            };

            let json = format!(
                r#"{{"stdout":"{}","state":"{}","duration_us":{}}}"#,
                escape_json_string(&res.stdout),
                state_str,
                res.duration_us
            );
            send_json_response(&mut stream, 200, &json, is_head);
        }
        ("OPTIONS", _) => {
            // Handle CORS preflight
            let response = "HTTP/1.1 204 No Content\r\n\
                Access-Control-Allow-Origin: *\r\n\
                Access-Control-Allow-Methods: GET, POST, OPTIONS, HEAD\r\n\
                Access-Control-Allow-Headers: Content-Type\r\n\r\n";
            stream.write_all(response.as_bytes()).ok();
        }
        _ => {
            let response = "HTTP/1.1 404 Not Found\r\nContent-Length: 9\r\n\r\nNot Found";
            stream.write_all(response.as_bytes()).ok();
        }
    }
}

fn serve_file(stream: &mut TcpStream, file_path: &str, content_type: &str, is_head: bool) {
    let candidates = [
        file_path.to_string(),
        format!("../<TRACK 3> : {{Applications & OmniStudio Interfaces}}/{file_path}"),
        format!("<TRACK 3> : {{Applications & OmniStudio Interfaces}}/{file_path}"),
        format!("apps/{file_path}"),
        format!("../<TRACK 3> : {{Applications & OmniStudio Interfaces}}/apps/{file_path}"),
        format!("<TRACK 3> : {{Applications & OmniStudio Interfaces}}/apps/{file_path}"),
    ];
    let resolved_path = candidates
        .iter()
        .find(|c| Path::new(c).exists())
        .cloned()
        .unwrap_or_else(|| file_path.to_string());

    if !Path::new(&resolved_path).exists() {
        let not_found = format!(
            "HTTP/1.1 404 Not Found\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\nFile Not Found: {}",
            resolved_path.len() + 16, resolved_path
        );
        stream.write_all(not_found.as_bytes()).ok();
        return;
    }

    match fs::read(&resolved_path) {
        Ok(data) => {
            let header = format!(
                "HTTP/1.1 200 OK\r\n\
                Content-Type: {}\r\n\
                Content-Length: {}\r\n\
                Access-Control-Allow-Origin: *\r\n\
                Connection: close\r\n\r\n",
                content_type, data.len()
            );
            stream.write_all(header.as_bytes()).ok();
            if !is_head {
                stream.write_all(&data).ok();
            }
        }
        Err(e) => {
            let err_body = format!("Error reading file: {e}");
            let header = format!(
                "HTTP/1.1 500 Internal Server Error\r\nContent-Length: {}\r\n\r\n{}",
                err_body.len(), err_body
            );
            stream.write_all(header.as_bytes()).ok();
        }
    }
}

fn send_json_response(stream: &mut TcpStream, status_code: u16, json: &str, is_head: bool) {
    let status_line = match status_code {
        200 => "200 OK",
        400 => "400 Bad Request",
        404 => "404 Not Found",
        500 => "500 Internal Server Error",
        _ => "200 OK",
    };

    let header = format!(
        "HTTP/1.1 {}\r\n\
        Content-Type: application/json; charset=utf-8\r\n\
        Content-Length: {}\r\n\
        Access-Control-Allow-Origin: *\r\n\
        Connection: close\r\n\r\n",
        status_line, json.as_bytes().len()
    );
    stream.write_all(header.as_bytes()).ok();
    if !is_head {
        stream.write_all(json.as_bytes()).ok();
    }
}

fn escape_json_string(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 10);
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            '\u{0008}' => out.push_str("\\b"),
            '\u{000C}' => out.push_str("\\f"),
            c if c.is_control() => {
                out.push_str(&format!("\\u{:04x}", c as u32));
            }
            c => out.push(c),
        }
    }
    out
}

fn extract_json_field(json: &str, field_name: &str) -> Option<String> {
    let key = format!("\"{}\":", field_name);
    let key_pos = json.find(&key)?;
    let remainder = json[key_pos + key.len()..].trim_start();

    if remainder.starts_with('"') {
        let mut result = String::new();
        let chars = remainder[1..].chars();
        let mut escaped = false;
        for c in chars {
            if escaped {
                match c {
                    '"' => result.push('"'),
                    '\\' => result.push('\\'),
                    'n' => result.push('\n'),
                    'r' => result.push('\r'),
                    't' => result.push('\t'),
                    other => result.push(other),
                }
                escaped = false;
            } else if c == '\\' {
                escaped = true;
            } else if c == '"' {
                return Some(result);
            } else {
                result.push(c);
            }
        }
        Some(result)
    } else {
        None
    }
}

fn extract_query_param(raw_path: &str, param: &str) -> Option<String> {
    let query_str = raw_path.split('?').nth(1)?;
    for pair in query_str.split('&') {
        let mut kv = pair.split('=');
        let k = kv.next()?;
        if k == param {
            let v = kv.next().unwrap_or("");
            return Some(v.replace("%20", " ").replace('+', " "));
        }
    }
    None
}

#[allow(dead_code)]
fn tokenize_args(input: &str) -> Vec<String> {
    let mut args = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;
    let mut quote_char = '"';

    for c in input.chars() {
        if in_quotes {
            if c == quote_char {
                in_quotes = false;
            } else {
                current.push(c);
            }
        } else if c == '"' || c == '\'' {
            in_quotes = true;
            quote_char = c;
        } else if c.is_whitespace() {
            if !current.is_empty() {
                args.push(current);
                current = String::new();
            }
        } else {
            current.push(c);
        }
    }
    if !current.is_empty() {
        args.push(current);
    }
    args
}


// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Server Module Status & Invariant Verification )
// ==============================================================================
// OmniCode GUI Server module sealed and active.
