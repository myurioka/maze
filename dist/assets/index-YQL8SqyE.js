(function(){const e=document.createElement("link").relList;if(e&&e.supports&&e.supports("modulepreload"))return;for(const r of document.querySelectorAll('link[rel="modulepreload"]'))o(r);new MutationObserver(r=>{for(const i of r)if(i.type==="childList")for(const s of i.addedNodes)s.tagName==="LINK"&&s.rel==="modulepreload"&&o(s)}).observe(document,{childList:!0,subtree:!0});function t(r){const i={};return r.integrity&&(i.integrity=r.integrity),r.referrerPolicy&&(i.referrerPolicy=r.referrerPolicy),r.crossOrigin==="use-credentials"?i.credentials="include":r.crossOrigin==="anonymous"?i.credentials="omit":i.credentials="same-origin",i}function o(r){if(r.ep)return;r.ep=!0;const i=t(r);fetch(r.href,i)}})();let _;const b=new Array(128).fill(void 0);b.push(void 0,null,!0,!1);function c(n){return b[n]}let m=b.length;function C(n){n<132||(b[n]=m,m=n)}function k(n){const e=c(n);return C(n),e}const M=typeof TextDecoder<"u"?new TextDecoder("utf-8",{ignoreBOM:!0,fatal:!0}):{decode:()=>{throw Error("TextDecoder not available")}};typeof TextDecoder<"u"&&M.decode();let y=null;function O(){return(y===null||y.byteLength===0)&&(y=new Uint8Array(_.memory.buffer)),y}function a(n,e){return n=n>>>0,M.decode(O().subarray(n,n+e))}function f(n){m===b.length&&b.push(b.length+1);const e=m;return m=b[e],b[e]=n,e}function S(n){const e=typeof n;if(e=="number"||e=="boolean"||n==null)return`${n}`;if(e=="string")return`"${n}"`;if(e=="symbol"){const r=n.description;return r==null?"Symbol":`Symbol(${r})`}if(e=="function"){const r=n.name;return typeof r=="string"&&r.length>0?`Function(${r})`:"Function"}if(Array.isArray(n)){const r=n.length;let i="[";r>0&&(i+=S(n[0]));for(let s=1;s<r;s++)i+=", "+S(n[s]);return i+="]",i}const t=/\[object ([^\]]+)\]/.exec(toString.call(n));let o;if(t.length>1)o=t[1];else return toString.call(n);if(o=="Object")try{return"Object("+JSON.stringify(n)+")"}catch{return"Object"}return n instanceof Error?`${n.name}: ${n.message}
${n.stack}`:o}let p=0;const A=typeof TextEncoder<"u"?new TextEncoder("utf-8"):{encode:()=>{throw Error("TextEncoder not available")}},I=typeof A.encodeInto=="function"?function(n,e){return A.encodeInto(n,e)}:function(n,e){const t=A.encode(n);return e.set(t),{read:n.length,written:t.length}};function T(n,e,t){if(t===void 0){const u=A.encode(n),w=e(u.length,1)>>>0;return O().subarray(w,w+u.length).set(u),p=u.length,w}let o=n.length,r=e(o,1)>>>0;const i=O();let s=0;for(;s<o;s++){const u=n.charCodeAt(s);if(u>127)break;i[r+s]=u}if(s!==o){s!==0&&(n=n.slice(s)),r=t(r,o,o=s+n.length*3,1)>>>0;const u=O().subarray(r+s,r+o),w=I(n,u);s+=w.written,r=t(r,o,s,1)>>>0}return p=s,r}let d=null;function l(){return(d===null||d.buffer.detached===!0||d.buffer.detached===void 0&&d.buffer!==_.memory.buffer)&&(d=new DataView(_.memory.buffer)),d}const E=typeof FinalizationRegistry>"u"?{register:()=>{},unregister:()=>{}}:new FinalizationRegistry(n=>{_.__wbindgen_export_2.get(n.dtor)(n.a,n.b)});function x(n,e,t,o){const r={a:n,b:e,cnt:1,dtor:t},i=(...s)=>{r.cnt++;const u=r.a;r.a=0;try{return o(u,r.b,...s)}finally{--r.cnt===0?(_.__wbindgen_export_2.get(r.dtor)(u,r.b),E.unregister(r)):r.a=u}};return i.original=r,E.register(i,r,r),i}function j(n,e,t){_._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__hc64dff1bf431a1df(n,e,f(t))}function W(n,e,t){_._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h7e392eaa67d9ab01(n,e,t)}function F(n,e,t){_._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h71194c68a4f183ac(n,e,f(t))}function L(){try{const t=_.__wbindgen_add_to_stack_pointer(-16);_.main(t);var n=l().getInt32(t+4*0,!0),e=l().getInt32(t+4*1,!0);if(e)throw k(n)}finally{_.__wbindgen_add_to_stack_pointer(16)}}function h(n){return n==null}function g(n,e){try{return n.apply(this,e)}catch(t){_.__wbindgen_exn_store(f(t))}}async function D(n,e){if(typeof Response=="function"&&n instanceof Response){if(typeof WebAssembly.instantiateStreaming=="function")try{return await WebAssembly.instantiateStreaming(n,e)}catch(o){if(n.headers.get("Content-Type")!="application/wasm")console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n",o);else throw o}const t=await n.arrayBuffer();return await WebAssembly.instantiate(t,e)}else{const t=await WebAssembly.instantiate(n,e);return t instanceof WebAssembly.Instance?{instance:t,module:n}:t}}function P(){const n={};return n.wbg={},n.wbg.__wbindgen_object_drop_ref=function(e){k(e)},n.wbg.__wbindgen_string_new=function(e,t){const o=a(e,t);return f(o)},n.wbg.__wbindgen_cb_drop=function(e){const t=k(e).original;return t.cnt--==1?(t.a=0,!0):!1},n.wbg.__wbg_new_abda76e883ba8a5f=function(){const e=new Error;return f(e)},n.wbg.__wbg_stack_658279fe44541cf6=function(e,t){const o=c(t).stack,r=T(o,_.__wbindgen_malloc,_.__wbindgen_realloc),i=p;l().setInt32(e+4*1,i,!0),l().setInt32(e+4*0,r,!0)},n.wbg.__wbg_error_f851667af71bcfc6=function(e,t){let o,r;try{o=e,r=t,console.error(a(e,t))}finally{_.__wbindgen_free(o,r,1)}},n.wbg.__wbg_instanceof_Window_f401953a2cf86220=function(e){let t;try{t=c(e)instanceof Window}catch{t=!1}return t},n.wbg.__wbg_document_5100775d18896c16=function(e){const t=c(e).document;return h(t)?0:f(t)},n.wbg.__wbg_performance_3298a9628a5c8aa4=function(e){const t=c(e).performance;return h(t)?0:f(t)},n.wbg.__wbg_requestAnimationFrame_549258cfa66011f0=function(){return g(function(e,t){return c(e).requestAnimationFrame(c(t))},arguments)},n.wbg.__wbg_getElementById_c369ff43f0db99cf=function(e,t,o){const r=c(e).getElementById(a(t,o));return h(r)?0:f(r)},n.wbg.__wbg_setonkeydown_7cb32e3de434af04=function(e,t){c(e).onkeydown=c(t)},n.wbg.__wbg_setonkeyup_b4ef1e42cfd68bb7=function(e,t){c(e).onkeyup=c(t)},n.wbg.__wbg_log_5bb5f88f245d7762=function(e){console.log(c(e))},n.wbg.__wbg_instanceof_CanvasRenderingContext2d_20bf99ccc051643b=function(e){let t;try{t=c(e)instanceof CanvasRenderingContext2D}catch{t=!1}return t},n.wbg.__wbg_setstrokeStyle_c79ba6bc36a7f302=function(e,t){c(e).strokeStyle=c(t)},n.wbg.__wbg_setfillStyle_4de94b275f5761f2=function(e,t){c(e).fillStyle=c(t)},n.wbg.__wbg_setfont_a4d031cf2c94b4db=function(e,t,o){c(e).font=a(t,o)},n.wbg.__wbg_settextAlign_d4f121248c40b910=function(e,t,o){c(e).textAlign=a(t,o)},n.wbg.__wbg_beginPath_c7b9e681f2d031ca=function(e){c(e).beginPath()},n.wbg.__wbg_fill_7f376d2e52c3054e=function(e){c(e).fill()},n.wbg.__wbg_stroke_b125233fc8b11e59=function(e){c(e).stroke()},n.wbg.__wbg_closePath_1e01ade2e4928be9=function(e){c(e).closePath()},n.wbg.__wbg_lineTo_863448482ad2bd29=function(e,t,o){c(e).lineTo(t,o)},n.wbg.__wbg_moveTo_5526d0fa563650fa=function(e,t,o){c(e).moveTo(t,o)},n.wbg.__wbg_clearRect_05de681275dda635=function(e,t,o,r,i){c(e).clearRect(t,o,r,i)},n.wbg.__wbg_fillText_6dfde0e3b04c85db=function(){return g(function(e,t,o,r,i){c(e).fillText(a(t,o),r,i)},arguments)},n.wbg.__wbg_code_3b0c3912a2351163=function(e,t){const o=c(t).code,r=T(o,_.__wbindgen_malloc,_.__wbindgen_realloc),i=p;l().setInt32(e+4*1,i,!0),l().setInt32(e+4*0,r,!0)},n.wbg.__wbg_now_4e659b3d15f470d9=function(e){return c(e).now()},n.wbg.__wbg_instanceof_HtmlCanvasElement_46bdbf323b0b18d1=function(e){let t;try{t=c(e)instanceof HTMLCanvasElement}catch{t=!1}return t},n.wbg.__wbg_getContext_df50fa48a8876636=function(){return g(function(e,t,o){const r=c(e).getContext(a(t,o));return h(r)?0:f(r)},arguments)},n.wbg.__wbg_queueMicrotask_48421b3cc9052b68=function(e){const t=c(e).queueMicrotask;return f(t)},n.wbg.__wbindgen_is_function=function(e){return typeof c(e)=="function"},n.wbg.__wbg_queueMicrotask_12a30234db4045d3=function(e){queueMicrotask(c(e))},n.wbg.__wbg_newnoargs_76313bd6ff35d0f2=function(e,t){const o=new Function(a(e,t));return f(o)},n.wbg.__wbg_call_1084a111329e68ce=function(){return g(function(e,t){const o=c(e).call(c(t));return f(o)},arguments)},n.wbg.__wbindgen_object_clone_ref=function(e){const t=c(e);return f(t)},n.wbg.__wbg_self_3093d5d1f7bcb682=function(){return g(function(){const e=self.self;return f(e)},arguments)},n.wbg.__wbg_window_3bcfc4d31bc012f8=function(){return g(function(){const e=window.window;return f(e)},arguments)},n.wbg.__wbg_globalThis_86b222e13bdf32ed=function(){return g(function(){const e=globalThis.globalThis;return f(e)},arguments)},n.wbg.__wbg_global_e5a3fe56f8be9485=function(){return g(function(){const e=global.global;return f(e)},arguments)},n.wbg.__wbindgen_is_undefined=function(e){return c(e)===void 0},n.wbg.__wbg_resolve_570458cb99d56a43=function(e){const t=Promise.resolve(c(e));return f(t)},n.wbg.__wbg_then_95e6edc0f89b73b1=function(e,t){const o=c(e).then(c(t));return f(o)},n.wbg.__wbindgen_debug_string=function(e,t){const o=S(c(t)),r=T(o,_.__wbindgen_malloc,_.__wbindgen_realloc),i=p;l().setInt32(e+4*1,i,!0),l().setInt32(e+4*0,r,!0)},n.wbg.__wbindgen_throw=function(e,t){throw new Error(a(e,t))},n.wbg.__wbindgen_closure_wrapper83=function(e,t,o){const r=x(e,t,21,j);return f(r)},n.wbg.__wbindgen_closure_wrapper85=function(e,t,o){const r=x(e,t,21,W);return f(r)},n.wbg.__wbindgen_closure_wrapper521=function(e,t,o){const r=x(e,t,81,F);return f(r)},n}function q(n,e){return _=n.exports,R.__wbindgen_wasm_module=e,d=null,y=null,_}async function R(n){if(_!==void 0)return _;typeof n<"u"&&Object.getPrototypeOf(n)===Object.prototype?{module_or_path:n}=n:console.warn("using deprecated parameters for the initialization function; pass a single object instead"),typeof n>"u"&&(n=new URL("/assets/wasm_bg-DyNptLjQ.wasm",import.meta.url));const e=P();(typeof n=="string"||typeof Request=="function"&&n instanceof Request||typeof URL=="function"&&n instanceof URL)&&(n=fetch(n));const{instance:t,module:o}=await D(await n,e);return q(t,o)}R().then(()=>L());