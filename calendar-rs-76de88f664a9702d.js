let W=0,_=`string`,S=1,$=`Object`,U=`utf-8`,R=null,Z=`number`,T=`undefined`,a0=4,Y=`function`,a1=16,a6=126,P=Array,a4=Date,V=Error,a3=Object,a5=Promise,a2=Reflect,X=Uint8Array,Q=undefined;var F=((b,c,d,f)=>{a.wasm_bindgen__convert__closures__invoke2_mut__h150aab9901250cf4(b,c,e(d),e(f))});var w=((b,c)=>{a.wasm_bindgen__convert__closures__invoke0_mut__hc85cbcdd29f7575f(b,c)});var u=(a=>{const b=typeof a;if(b==Z||b==`boolean`||a==R){return `${a}`};if(b==_){return `"${a}"`};if(b==`symbol`){const b=a.description;if(b==R){return `Symbol`}else{return `Symbol(${b})`}};if(b==Y){const b=a.name;if(typeof b==_&&b.length>W){return `Function(${b})`}else{return `Function`}};if(P.isArray(a)){const b=a.length;let c=`[`;if(b>W){c+=u(a[W])};for(let d=S;d<b;d++){c+=`, `+ u(a[d])};c+=`]`;return c};const c=/\[object ([^\]]+)\]/.exec(toString.call(a));let d;if(c.length>S){d=c[S]}else{return toString.call(a)};if(d==$){try{return `Object(`+ JSON.stringify(a)+ `)`}catch(a){return $}};if(a instanceof V){return `${a.name}: ${a.message}\n${a.stack}`};return d});var L=((a,b)=>{});var E=((a,b)=>{a=a>>>W;const c=D();const d=c.subarray(a/a0,a/a0+ b);const e=[];for(let a=W;a<d.length;a++){e.push(g(d[a]))};return e});var x=((b,c,d)=>{a.wasm_bindgen__convert__closures__invoke1_mut__hb5f328ff892b2d87(b,c,e(d))});var g=(a=>{const b=c(a);f(a);return b});var e=(a=>{if(d===b.length)b.push(b.length+ S);const c=d;d=b[c];b[c]=a;return c});var O=(async(b)=>{if(a!==Q)return a;if(typeof b===T){b=new URL(`calendar-rs-76de88f664a9702d_bg.wasm`,import.meta.url)};const c=K();if(typeof b===_||typeof Request===Y&&b instanceof Request||typeof URL===Y&&b instanceof URL){b=fetch(b)};L(c);const {instance:d,module:e}=await J(await b,c);return M(d,e)});var y=((b,c,d)=>{a.wasm_bindgen__convert__closures__invoke1_mut__ha4d7feaf4239826c(b,c,e(d))});function B(b,c){try{return b.apply(this,c)}catch(b){a.__wbindgen_exn_store(e(b))}}var A=((a,b)=>{if(a===W){return c(b)}else{return k(a,b)}});var N=(b=>{if(a!==Q)return a;const c=K();L(c);if(!(b instanceof WebAssembly.Module)){b=new WebAssembly.Module(b)};const d=new WebAssembly.Instance(b,c);return M(d,b)});var p=(()=>{if(o===R||o.byteLength===W){o=new Int32Array(a.memory.buffer)};return o});var l=(a=>a===Q||a===R);var c=(a=>b[a]);var M=((b,c)=>{a=b.exports;O.__wbindgen_wasm_module=c;m=R;o=R;C=R;i=R;a.__wbindgen_start();return a});var K=(()=>{const b={};b.wbg={};b.wbg.__wbindgen_object_clone_ref=(a=>{const b=c(a);return e(b)});b.wbg.__wbindgen_object_drop_ref=(a=>{g(a)});b.wbg.__wbindgen_cb_drop=(a=>{const b=g(a).original;if(b.cnt--==S){b.a=W;return !0};const c=!1;return c});b.wbg.__wbindgen_string_new=((a,b)=>{const c=k(a,b);return e(c)});b.wbg.__wbindgen_number_get=((a,b)=>{const d=c(b);const e=typeof d===Z?d:Q;n()[a/8+ S]=l(e)?W:e;p()[a/a0+ W]=!l(e)});b.wbg.__wbindgen_is_undefined=(a=>{const b=c(a)===Q;return b});b.wbg.__wbg_new_abda76e883ba8a5f=(()=>{const a=new V();return e(a)});b.wbg.__wbg_stack_658279fe44541cf6=((b,d)=>{const e=c(d).stack;const f=t(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=q;p()[b/a0+ S]=g;p()[b/a0+ W]=f});b.wbg.__wbg_error_f851667af71bcfc6=((b,c)=>{var d=A(b,c);if(b!==W){a.__wbindgen_free(b,c,S)};console.error(d)});b.wbg.__wbg_log_53ed96ea72ace5e9=((a,b)=>{var c=A(a,b);console.log(c)});b.wbg.__wbg_error_93b671ae91baaee7=((a,b)=>{var c=A(a,b);console.error(c)});b.wbg.__wbg_warn_52c5b3e773c3a056=((a,b)=>{var c=A(a,b);console.warn(c)});b.wbg.__wbindgen_is_null=(a=>{const b=c(a)===R;return b});b.wbg.__wbindgen_is_falsy=(a=>{const b=!c(a);return b});b.wbg.__wbindgen_number_new=(a=>{const b=a;return e(b)});b.wbg.__wbg_setTimeout_7d81d052875b0f4f=function(){return B(((a,b)=>{const d=setTimeout(c(a),b);return e(d)}),arguments)};b.wbg.__wbindgen_string_get=((b,d)=>{const e=c(d);const f=typeof e===_?e:Q;var g=l(f)?W:t(f,a.__wbindgen_malloc,a.__wbindgen_realloc);var h=q;p()[b/a0+ S]=h;p()[b/a0+ W]=g});b.wbg.__wbg_queueMicrotask_26a89c14c53809c0=(a=>{const b=c(a).queueMicrotask;return e(b)});b.wbg.__wbindgen_is_function=(a=>{const b=typeof c(a)===Y;return b});b.wbg.__wbg_queueMicrotask_118eeb525d584d9a=(a=>{queueMicrotask(c(a))});b.wbg.__wbg_info_308c715c6cec230d=((b,c)=>{var d=E(b,c).slice();a.__wbindgen_free(b,c*a0,a0);console.info(...d)});b.wbg.__wbg_instanceof_Window_99dc9805eaa2614b=(a=>{let b;try{b=c(a) instanceof Window}catch(a){b=!1}const d=b;return d});b.wbg.__wbg_document_5257b70811e953c0=(a=>{const b=c(a).document;return l(b)?W:e(b)});b.wbg.__wbg_innerHeight_dc4c81e04e8bc294=function(){return B((a=>{const b=c(a).innerHeight;return e(b)}),arguments)};b.wbg.__wbg_fetch_ba9c8f5d941ae5c4=((a,b)=>{const d=c(a).fetch(c(b));return e(d)});b.wbg.__wbg_body_3eb73da919b867a1=(a=>{const b=c(a).body;return l(b)?W:e(b)});b.wbg.__wbg_createComment_ce9f467394242d45=((a,b,d)=>{var f=A(b,d);const g=c(a).createComment(f);return e(g)});b.wbg.__wbg_createDocumentFragment_229f723f44e69ab9=(a=>{const b=c(a).createDocumentFragment();return e(b)});b.wbg.__wbg_createElement_1a136faad4101f43=function(){return B(((a,b,d)=>{var f=A(b,d);const g=c(a).createElement(f);return e(g)}),arguments)};b.wbg.__wbg_createElementNS_d47e0c50fa2904e0=function(){return B(((a,b,d,f,g)=>{var h=A(b,d);var i=A(f,g);const j=c(a).createElementNS(h,i);return e(j)}),arguments)};b.wbg.__wbg_createTextNode_dbdd908f92bae1b1=((a,b,d)=>{var f=A(b,d);const g=c(a).createTextNode(f);return e(g)});b.wbg.__wbg_classList_b75072943b838f29=(a=>{const b=c(a).classList;return e(b)});b.wbg.__wbg_setinnerHTML_99deeacfff0ae4cc=((a,b,d)=>{var e=A(b,d);c(a).innerHTML=e});b.wbg.__wbg_removeAttribute_5c264e727b67dbdb=function(){return B(((a,b,d)=>{var e=A(b,d);c(a).removeAttribute(e)}),arguments)};b.wbg.__wbg_scroll_472215eda06dccba=((a,b)=>{c(a).scroll(c(b))});b.wbg.__wbg_setAttribute_0918ea45d5a1c663=function(){return B(((a,b,d,e,f)=>{var g=A(b,d);var h=A(e,f);c(a).setAttribute(g,h)}),arguments)};b.wbg.__wbg_before_bed7b7b6e53dd469=function(){return B(((a,b)=>{c(a).before(c(b))}),arguments)};b.wbg.__wbg_remove_ed2f62f1a8be044b=(a=>{c(a).remove()});b.wbg.__wbg_append_459bddb5f3a5b5fa=function(){return B(((a,b)=>{c(a).append(c(b))}),arguments)};b.wbg.__wbg_scrollTop_b8364983aece464a=(a=>{const b=c(a).scrollTop;return b});b.wbg.__wbg_setscrollTop_cdd1f96b8da164d5=((a,b)=>{c(a).scrollTop=b});b.wbg.__wbg_add_44212bfb95df48ba=function(){return B(((a,b,d)=>{var e=A(b,d);c(a).add(e)}),arguments)};b.wbg.__wbg_remove_0df84ff63b459921=function(){return B(((a,b,d)=>{var e=A(b,d);c(a).remove(e)}),arguments)};b.wbg.__wbg_new_a979e9eedc5e81a3=function(){return B((()=>{const a=new Headers();return e(a)}),arguments)};b.wbg.__wbg_parentNode_f3957fdd408a62f7=(a=>{const b=c(a).parentNode;return l(b)?W:e(b)});b.wbg.__wbg_childNodes_75d3da5f3a7bb985=(a=>{const b=c(a).childNodes;return e(b)});b.wbg.__wbg_previousSibling_4cd9e84aeb4df529=(a=>{const b=c(a).previousSibling;return l(b)?W:e(b)});b.wbg.__wbg_nextSibling_13e9454ef5323f1a=(a=>{const b=c(a).nextSibling;return l(b)?W:e(b)});b.wbg.__wbg_settextContent_1fec240f77aa3dc4=((a,b,d)=>{var e=A(b,d);c(a).textContent=e});b.wbg.__wbg_appendChild_bd383ec5356c0bdb=function(){return B(((a,b)=>{const d=c(a).appendChild(c(b));return e(d)}),arguments)};b.wbg.__wbg_cloneNode_80501c66ab115588=function(){return B((a=>{const b=c(a).cloneNode();return e(b)}),arguments)};b.wbg.__wbg_search_b5c7b044aaf64616=((b,d)=>{const e=c(d).search;const f=t(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=q;p()[b/a0+ S]=g;p()[b/a0+ W]=f});b.wbg.__wbg_setsearch_ad0620e22e67a913=((a,b,d)=>{var e=A(b,d);c(a).search=e});b.wbg.__wbg_new_7d30e9d9d2deaf9d=function(){return B(((a,b)=>{var c=A(a,b);const d=new URL(c);return e(d)}),arguments)};b.wbg.__wbg_addEventListener_2f891d22985fd3c8=function(){return B(((a,b,d,e)=>{var f=A(b,d);c(a).addEventListener(f,c(e))}),arguments)};b.wbg.__wbg_addEventListener_1b158e9e95e0ab00=function(){return B(((a,b,d,e,f)=>{var g=A(b,d);c(a).addEventListener(g,c(e),c(f))}),arguments)};b.wbg.__wbg_length_d5ed87010607a669=(a=>{const b=c(a).length;return b});b.wbg.__wbg_instanceof_ShadowRoot_cb6366cb0956ce29=(a=>{let b;try{b=c(a) instanceof ShadowRoot}catch(a){b=!1}const d=b;return d});b.wbg.__wbg_host_99e27ed8897850f2=(a=>{const b=c(a).host;return e(b)});b.wbg.__wbg_url_70f3179afe0eccd6=((b,d)=>{const e=c(d).url;const f=t(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=q;p()[b/a0+ S]=g;p()[b/a0+ W]=f});b.wbg.__wbg_newwithstr_19bf69d1840d2816=function(){return B(((a,b)=>{var c=A(a,b);const d=new Request(c);return e(d)}),arguments)};b.wbg.__wbg_newwithstrandinit_9fd2fc855c6327eb=function(){return B(((a,b,d)=>{var f=A(a,b);const g=new Request(f,c(d));return e(g)}),arguments)};b.wbg.__wbg_instanceof_Response_0d25bb8436a9cefe=(a=>{let b;try{b=c(a) instanceof Response}catch(a){b=!1}const d=b;return d});b.wbg.__wbg_text_10c88c5e55f873c7=function(){return B((a=>{const b=c(a).text();return e(b)}),arguments)};b.wbg.__wbg_new_26bb7e688dfc365c=function(){return B((()=>{const a=new URLSearchParams();return e(a)}),arguments)};b.wbg.__wbg_view_d7afa0120e493b2d=(a=>{const b=c(a).view;return l(b)?W:e(b)});b.wbg.__wbg_respond_3233ecfa19b9b617=function(){return B(((a,b)=>{c(a).respond(b>>>W)}),arguments)};b.wbg.__wbg_close_21d8fce01634cc74=function(){return B((a=>{c(a).close()}),arguments)};b.wbg.__wbg_enqueue_61ebfae3475d5d91=function(){return B(((a,b)=>{c(a).enqueue(c(b))}),arguments)};b.wbg.__wbg_setdata_4d5b377238fff97c=((a,b,d)=>{var e=A(b,d);c(a).data=e});b.wbg.__wbg_target_791826e938c3e308=(a=>{const b=c(a).target;return l(b)?W:e(b)});b.wbg.__wbg_cancelBubble_191799b8e0ab3254=(a=>{const b=c(a).cancelBubble;return b});b.wbg.__wbg_composedPath_d94a39b8c8f6eed1=(a=>{const b=c(a).composedPath();return e(b)});b.wbg.__wbg_error_1f4e3e298a7c97f6=(a=>{console.error(c(a))});b.wbg.__wbg_warn_0e0204547af47087=(a=>{console.warn(c(a))});b.wbg.__wbg_new_4d5935236eea57e5=function(){return B((()=>{const a=new Range();return e(a)}),arguments)};b.wbg.__wbg_deleteContents_4ef35f6b1f6bdb41=function(){return B((a=>{c(a).deleteContents()}),arguments)};b.wbg.__wbg_setEndBefore_213652954c786b01=function(){return B(((a,b)=>{c(a).setEndBefore(c(b))}),arguments)};b.wbg.__wbg_setStartBefore_feaf6bd3922515df=function(){return B(((a,b)=>{c(a).setStartBefore(c(b))}),arguments)};b.wbg.__wbg_append_517583bac5b5bb16=function(){return B(((a,b)=>{c(a).append(c(b))}),arguments)};b.wbg.__wbg_byobRequest_004146c1db53bc14=(a=>{const b=c(a).byobRequest;return l(b)?W:e(b)});b.wbg.__wbg_close_54a5b70c42a72ee3=function(){return B((a=>{c(a).close()}),arguments)};b.wbg.__wbg_instanceof_WorkerGlobalScope_a9d2cb51ce9a4579=(a=>{let b;try{b=c(a) instanceof WorkerGlobalScope}catch(a){b=!1}const d=b;return d});b.wbg.__wbg_fetch_06d656a1b748ac0d=((a,b)=>{const d=c(a).fetch(c(b));return e(d)});b.wbg.__wbg_get_c43534c00f382c8a=((a,b)=>{const d=c(a)[b>>>W];return e(d)});b.wbg.__wbg_newnoargs_5859b6d41c6fe9f7=((a,b)=>{var c=A(a,b);const d=new Function(c);return e(d)});b.wbg.__wbg_get_5027b32da70f39b1=function(){return B(((a,b)=>{const d=a2.get(c(a),c(b));return e(d)}),arguments)};b.wbg.__wbg_call_a79f1973a4f07d5e=function(){return B(((a,b)=>{const d=c(a).call(c(b));return e(d)}),arguments)};b.wbg.__wbg_new_87d841e70661f6e9=(()=>{const a=new a3();return e(a)});b.wbg.__wbg_self_086b5302bcafb962=function(){return B((()=>{const a=self.self;return e(a)}),arguments)};b.wbg.__wbg_window_132fa5d7546f1de5=function(){return B((()=>{const a=window.window;return e(a)}),arguments)};b.wbg.__wbg_globalThis_e5f801a37ad7d07b=function(){return B((()=>{const a=globalThis.globalThis;return e(a)}),arguments)};b.wbg.__wbg_global_f9a61fce4af6b7c1=function(){return B((()=>{const a=global.global;return e(a)}),arguments)};b.wbg.__wbg_instanceof_Error_f5ae6a28929a8190=(a=>{let b;try{b=c(a) instanceof V}catch(a){b=!1}const d=b;return d});b.wbg.__wbg_new_3a66822ed076951c=((a,b)=>{var c=A(a,b);const d=new V(c);return e(d)});b.wbg.__wbg_message_5dbdf59ed61bbc49=(a=>{const b=c(a).message;return e(b)});b.wbg.__wbg_name_90a0336d27b12317=(a=>{const b=c(a).name;return e(b)});b.wbg.__wbg_toString_5326377607a05bf2=(a=>{const b=c(a).toString();return e(b)});b.wbg.__wbg_call_f6a2bc58c19c53c6=function(){return B(((a,b,d)=>{const f=c(a).call(c(b),c(d));return e(f)}),arguments)};b.wbg.__wbg_getTime_af7ca51c0bcefa08=(a=>{const b=c(a).getTime();return b});b.wbg.__wbg_getTimezoneOffset_98604170efd7a383=(a=>{const b=c(a).getTimezoneOffset();return b});b.wbg.__wbg_new_aaf6fa5a24e25a70=(a=>{const b=new a4(c(a));return e(b)});b.wbg.__wbg_new0_c0e40662db0749ee=(()=>{const a=new a4();return e(a)});b.wbg.__wbg_is_a5728dbfb61c82cd=((a,b)=>{const d=a3.is(c(a),c(b));return d});b.wbg.__wbg_toString_61d1ba76c783d2bc=(a=>{const b=c(a).toString();return e(b)});b.wbg.__wbg_new_1d93771b84541aa5=((a,b)=>{try{var c={a:a,b:b};var d=(a,b)=>{const d=c.a;c.a=W;try{return F(d,c.b,a,b)}finally{c.a=d}};const f=new a5(d);return e(f)}finally{c.a=c.b=W}});b.wbg.__wbg_resolve_97ecd55ee839391b=(a=>{const b=a5.resolve(c(a));return e(b)});b.wbg.__wbg_then_7aeb7c5f1536640f=((a,b)=>{const d=c(a).then(c(b));return e(d)});b.wbg.__wbg_then_5842e4e97f7beace=((a,b,d)=>{const f=c(a).then(c(b),c(d));return e(f)});b.wbg.__wbg_buffer_5d1b598a01b41a42=(a=>{const b=c(a).buffer;return e(b)});b.wbg.__wbg_newwithbyteoffsetandlength_d695c7957788f922=((a,b,d)=>{const f=new X(c(a),b>>>W,d>>>W);return e(f)});b.wbg.__wbg_set_74906aa30864df5a=((a,b,d)=>{c(a).set(c(b),d>>>W)});b.wbg.__wbg_length_f0764416ba5bb237=(a=>{const b=c(a).length;return b});b.wbg.__wbg_buffer_3da2aecfd9814cd8=(a=>{const b=c(a).buffer;return e(b)});b.wbg.__wbg_byteLength_a8d894d93425b2e0=(a=>{const b=c(a).byteLength;return b});b.wbg.__wbg_byteOffset_89d0a5265d5bde53=(a=>{const b=c(a).byteOffset;return b});b.wbg.__wbg_set_37a50e901587b477=function(){return B(((a,b,d)=>{const e=a2.set(c(a),c(b),c(d));return e}),arguments)};b.wbg.__wbindgen_debug_string=((b,d)=>{const e=u(c(d));const f=t(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=q;p()[b/a0+ S]=g;p()[b/a0+ W]=f});b.wbg.__wbindgen_throw=((a,b)=>{throw new V(k(a,b))});b.wbg.__wbindgen_memory=(()=>{const b=a.memory;return e(b)});b.wbg.__wbindgen_closure_wrapper383=((a,b,c)=>{const d=v(a,b,a6,w);return e(d)});b.wbg.__wbindgen_closure_wrapper385=((a,b,c)=>{const d=v(a,b,a6,x);return e(d)});b.wbg.__wbindgen_closure_wrapper387=((a,b,c)=>{const d=v(a,b,a6,x);return e(d)});b.wbg.__wbindgen_closure_wrapper695=((a,b,c)=>{const d=v(a,b,283,y);return e(d)});b.wbg.__wbindgen_closure_wrapper901=((a,b,c)=>{const d=v(a,b,343,z);return e(d)});return b});var z=((b,c,d)=>{a.wasm_bindgen__convert__closures__invoke1_mut__ha250071170c6b079(b,c,e(d))});var n=(()=>{if(m===R||m.byteLength===W){m=new Float64Array(a.memory.buffer)};return m});var D=(()=>{if(C===R||C.byteLength===W){C=new Uint32Array(a.memory.buffer)};return C});var f=(a=>{if(a<132)return;b[a]=d;d=a});var v=((b,c,d,e)=>{const f={a:b,b:c,cnt:S,dtor:d};const g=(...b)=>{f.cnt++;const c=f.a;f.a=W;try{return e(c,f.b,...b)}finally{if(--f.cnt===W){a.__wbindgen_export_2.get(f.dtor)(c,f.b)}else{f.a=c}}};g.original=f;return g});var t=((a,b,c)=>{if(c===Q){const c=r.encode(a);const d=b(c.length,S)>>>W;j().subarray(d,d+ c.length).set(c);q=c.length;return d};let d=a.length;let e=b(d,S)>>>W;const f=j();let g=W;for(;g<d;g++){const b=a.charCodeAt(g);if(b>127)break;f[e+ g]=b};if(g!==d){if(g!==W){a=a.slice(g)};e=c(e,d,d=g+ a.length*3,S)>>>W;const b=j().subarray(e+ g,e+ d);const f=s(a,b);g+=f.written};q=g;return e});var j=(()=>{if(i===R||i.byteLength===W){i=new X(a.memory.buffer)};return i});var k=((a,b)=>{a=a>>>W;return h.decode(j().subarray(a,a+ b))});var J=(async(a,b)=>{if(typeof Response===Y&&a instanceof Response){if(typeof WebAssembly.instantiateStreaming===Y){try{return await WebAssembly.instantiateStreaming(a,b)}catch(b){if(a.headers.get(`Content-Type`)!=`application/wasm`){console.warn(`\`WebAssembly.instantiateStreaming\` failed because your server does not serve wasm with \`application/wasm\` MIME type. Falling back to \`WebAssembly.instantiate\` which is slower. Original error:\\n`,b)}else{throw b}}};const c=await a.arrayBuffer();return await WebAssembly.instantiate(c,b)}else{const c=await WebAssembly.instantiate(a,b);if(c instanceof WebAssembly.Instance){return {instance:c,module:a}}else{return c}}});let a;const b=new P(128).fill(Q);b.push(Q,R,!0,!1);let d=b.length;const h=typeof TextDecoder!==T?new TextDecoder(U,{ignoreBOM:!0,fatal:!0}):{decode:()=>{throw V(`TextDecoder not available`)}};if(typeof TextDecoder!==T){h.decode()};let i=R;let m=R;let o=R;let q=W;const r=typeof TextEncoder!==T?new TextEncoder(U):{encode:()=>{throw V(`TextEncoder not available`)}};const s=typeof r.encodeInto===Y?((a,b)=>r.encodeInto(a,b)):((a,b)=>{const c=r.encode(a);b.set(c);return {read:a.length,written:c.length}});let C=R;class G{__destroy_into_raw(){const a=this.__wbg_ptr;this.__wbg_ptr=W;return a}free(){const b=this.__destroy_into_raw();a.__wbg_intounderlyingbytesource_free(b)}type(){try{const e=a.__wbindgen_add_to_stack_pointer(-a1);a.intounderlyingbytesource_type(e,this.__wbg_ptr);var b=p()[e/a0+ W];var c=p()[e/a0+ S];var d=A(b,c);if(b!==W){a.__wbindgen_free(b,c,S)};return d}finally{a.__wbindgen_add_to_stack_pointer(a1)}}autoAllocateChunkSize(){const b=a.intounderlyingbytesource_autoAllocateChunkSize(this.__wbg_ptr);return b>>>W}start(b){a.intounderlyingbytesource_start(this.__wbg_ptr,e(b))}pull(b){const c=a.intounderlyingbytesource_pull(this.__wbg_ptr,e(b));return g(c)}cancel(){const b=this.__destroy_into_raw();a.intounderlyingbytesource_cancel(b)}}class H{__destroy_into_raw(){const a=this.__wbg_ptr;this.__wbg_ptr=W;return a}free(){const b=this.__destroy_into_raw();a.__wbg_intounderlyingsink_free(b)}write(b){const c=a.intounderlyingsink_write(this.__wbg_ptr,e(b));return g(c)}close(){const b=this.__destroy_into_raw();const c=a.intounderlyingsink_close(b);return g(c)}abort(b){const c=this.__destroy_into_raw();const d=a.intounderlyingsink_abort(c,e(b));return g(d)}}class I{__destroy_into_raw(){const a=this.__wbg_ptr;this.__wbg_ptr=W;return a}free(){const b=this.__destroy_into_raw();a.__wbg_intounderlyingsource_free(b)}pull(b){const c=a.intounderlyingsource_pull(this.__wbg_ptr,e(b));return g(c)}cancel(){const b=this.__destroy_into_raw();a.intounderlyingsource_cancel(b)}}export default O;export{G as IntoUnderlyingByteSource,H as IntoUnderlyingSink,I as IntoUnderlyingSource,N as initSync}