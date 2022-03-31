(()=>{"use strict";var e,t,n,r,o,i,a,c,s,u,f,l,p,d,b,_,g,h,w,v,m,y,x,k,S,j,E,P,O,M={534:(e,t,n)=>{const r=Symbol("Comlink.proxy"),o=Symbol("Comlink.endpoint"),i=Symbol("Comlink.releaseProxy"),a=Symbol("Comlink.thrown"),c=e=>"object"==typeof e&&null!==e||"function"==typeof e,s=new Map([["proxy",{canHandle:e=>c(e)&&e[r],serialize(e){const{port1:t,port2:n}=new MessageChannel;return u(e,t),[n,[n]]},deserialize:e=>(e.start(),p(e,[],undefined))}],["throw",{canHandle:e=>c(e)&&a in e,serialize({value:e}){let t;return t=e instanceof Error?{isError:!0,value:{message:e.message,name:e.name,stack:e.stack}}:{isError:!1,value:e},[t,[]]},deserialize(e){if(e.isError)throw Object.assign(new Error(e.value.message),e.value);throw e.value}}]]);function u(e,t=self){t.addEventListener("message",(function n(o){if(!o||!o.data)return;const{id:i,type:c,path:s}=Object.assign({path:[]},o.data),l=(o.data.argumentList||[]).map(g);let p;try{const t=s.slice(0,-1).reduce(((e,t)=>e[t]),e),n=s.reduce(((e,t)=>e[t]),e);switch(c){case 0:p=n;break;case 1:t[s.slice(-1)[0]]=g(o.data.value),p=!0;break;case 2:p=n.apply(t,l);break;case 3:p=function(e){return Object.assign(e,{[r]:!0})}(new n(...l));break;case 4:{const{port1:t,port2:n}=new MessageChannel;u(e,n),p=function(e,t){return b.set(e,t),e}(t,[t])}break;case 5:p=void 0}}catch(e){p={value:e,[a]:0}}Promise.resolve(p).catch((e=>({value:e,[a]:0}))).then((e=>{const[r,o]=_(e);t.postMessage(Object.assign(Object.assign({},r),{id:i}),o),5===c&&(t.removeEventListener("message",n),f(t))}))})),t.start&&t.start()}function f(e){(function(e){return"MessagePort"===e.constructor.name})(e)&&e.close()}function l(e){if(e)throw new Error("Proxy has been released and is not useable")}function p(e,t=[],n=function(){}){let r=!1;const a=new Proxy(n,{get(n,o){if(l(r),o===i)return()=>h(e,{type:5,path:t.map((e=>e.toString()))}).then((()=>{f(e),r=!0}));if("then"===o){if(0===t.length)return{then:()=>a};const n=h(e,{type:0,path:t.map((e=>e.toString()))}).then(g);return n.then.bind(n)}return p(e,[...t,o])},set(n,o,i){l(r);const[a,c]=_(i);return h(e,{type:1,path:[...t,o].map((e=>e.toString())),value:a},c).then(g)},apply(n,i,a){l(r);const c=t[t.length-1];if(c===o)return h(e,{type:4}).then(g);if("bind"===c)return p(e,t.slice(0,-1));const[s,u]=d(a);return h(e,{type:2,path:t.map((e=>e.toString())),argumentList:s},u).then(g)},construct(n,o){l(r);const[i,a]=d(o);return h(e,{type:3,path:t.map((e=>e.toString())),argumentList:i},a).then(g)}});return a}function d(e){const t=e.map(_);return[t.map((e=>e[0])),(n=t.map((e=>e[1])),Array.prototype.concat.apply([],n))];var n}const b=new WeakMap;function _(e){for(const[t,n]of s)if(n.canHandle(e)){const[r,o]=n.serialize(e);return[{type:3,name:t,value:r},o]}return[{type:0,value:e},b.get(e)||[]]}function g(e){switch(e.type){case 3:return s.get(e.name).deserialize(e.value);case 0:return e.value}}function h(e,t,n){return new Promise((r=>{const o=new Array(4).fill(0).map((()=>Math.floor(Math.random()*Number.MAX_SAFE_INTEGER).toString(16))).join("-");e.addEventListener("message",(function t(n){n.data&&n.data.id&&n.data.id===o&&(e.removeEventListener("message",t),r(n.data))})),e.start&&e.start(),e.postMessage(Object.assign({id:o},t),n)}))}var w=function(e,t,n,r){return new(n||(n=Promise))((function(o,i){function a(e){try{s(r.next(e))}catch(e){i(e)}}function c(e){try{s(r.throw(e))}catch(e){i(e)}}function s(e){var t;e.done?o(e.value):(t=e.value,t instanceof n?t:new n((function(e){e(t)}))).then(a,c)}s((r=r.apply(e,t||[])).next())}))},v=function(e,t){var n,r,o,i,a={label:0,sent:function(){if(1&o[0])throw o[1];return o[1]},trys:[],ops:[]};return i={next:c(0),throw:c(1),return:c(2)},"function"==typeof Symbol&&(i[Symbol.iterator]=function(){return this}),i;function c(i){return function(c){return function(i){if(n)throw new TypeError("Generator is already executing.");for(;a;)try{if(n=1,r&&(o=2&i[0]?r.return:i[0]?r.throw||((o=r.return)&&o.call(r),0):r.next)&&!(o=o.call(r,i[1])).done)return o;switch(r=0,o&&(i=[2&i[0],o.value]),i[0]){case 0:case 1:o=i;break;case 4:return a.label++,{value:i[1],done:!1};case 5:a.label++,r=i[1],i=[0];continue;case 7:i=a.ops.pop(),a.trys.pop();continue;default:if(!((o=(o=a.trys).length>0&&o[o.length-1])||6!==i[0]&&2!==i[0])){a=0;continue}if(3===i[0]&&(!o||i[1]>o[0]&&i[1]<o[3])){a.label=i[1];break}if(6===i[0]&&a.label<o[1]){a.label=o[1],o=i;break}if(o&&a.label<o[2]){a.label=o[2],a.ops.push(i);break}o[2]&&a.ops.pop(),a.trys.pop();continue}i=t.call(e,a)}catch(e){i=[6,e],r=0}finally{n=o=0}if(5&i[0])throw i[1];return{value:i[0]?i[1]:void 0,done:!0}}([i,c])}}},m=[];u({init:function(e,t,r){return w(void 0,void 0,void 0,(function(){return v(this,(function(o){switch(o.label){case 0:return self.term={write:e,get_key:function(){var e=m.shift();return null==e?void 0:e.key},get_col_size:function(){return t},get_row_size:function(){return r}},[4,n.e(235).then(n.bind(n,235))];case 1:return o.sent(),[2]}}))}))},send_key:function(e,t){return w(void 0,void 0,void 0,(function(){return v(this,(function(n){return m.push({key:e,ctrl:t}),[2]}))}))}})}},A={};function z(e){var t=A[e];if(void 0!==t)return t.exports;var n=A[e]={id:e,loaded:!1,exports:{}};return M[e](n,n.exports,z),n.loaded=!0,n.exports}z.m=M,z.c=A,z.d=(e,t)=>{for(var n in t)z.o(t,n)&&!z.o(e,n)&&Object.defineProperty(e,n,{enumerable:!0,get:t[n]})},z.f={},z.e=e=>Promise.all(Object.keys(z.f).reduce(((t,n)=>(z.f[n](e,t),t)),[])),z.u=e=>e+".js",z.g=function(){if("object"==typeof globalThis)return globalThis;try{return this||new Function("return this")()}catch(e){if("object"==typeof window)return window}}(),z.hmd=e=>((e=Object.create(e)).children||(e.children=[]),Object.defineProperty(e,"exports",{enumerable:!0,set:()=>{throw new Error("ES Modules may not assign module.exports or exports.*, Use ESM export syntax, instead: "+e.id)}}),e),z.o=(e,t)=>Object.prototype.hasOwnProperty.call(e,t),z.r=e=>{"undefined"!=typeof Symbol&&Symbol.toStringTag&&Object.defineProperty(e,Symbol.toStringTag,{value:"Module"}),Object.defineProperty(e,"__esModule",{value:!0})},(()=>{var e;z.g.importScripts&&(e=z.g.location+"");var t=z.g.document;if(!e&&t&&(t.currentScript&&(e=t.currentScript.src),!e)){var n=t.getElementsByTagName("script");n.length&&(e=n[n.length-1].src)}if(!e)throw new Error("Automatic publicPath is not supported in this browser");e=e.replace(/#.*$/,"").replace(/\?.*$/,"").replace(/\/[^\/]+$/,"/"),z.p=e})(),(()=>{var e={872:1};z.f.i=(t,n)=>{e[t]||importScripts(z.p+z.u(t))};var t=self.webpackChunkrust_webpack_template=self.webpackChunkrust_webpack_template||[],n=t.push.bind(t);t.push=t=>{var[r,o,i]=t;for(var a in o)z.o(o,a)&&(z.m[a]=o[a]);for(i&&i(z);r.length;)e[r.pop()]=1;n(t)}})(),E={},P={716:function(){return{"./index_bg.js":{__wbg_getcolsize_a10523f79965babc:function(){return void 0===e&&(e=z.c[508].exports),e.$V()},__wbg_getrowsize_f108190ff429e4d8:function(){return void 0===t&&(t=z.c[508].exports),t.Qb()},__wbg_timeout_6453e5d4636904c4:function(){return void 0===n&&(n=z.c[508].exports),n.sJ()},__wbindgen_cb_drop:function(e){return void 0===r&&(r=z.c[508].exports),r.G6(e)},__wbg_write_daa581465730aee1:function(e,t){return void 0===o&&(o=z.c[508].exports),o.eV(e,t)},__wbg_getkey_17de46949815a52d:function(e){return void 0===i&&(i=z.c[508].exports),i.Xl(e)},__wbindgen_object_drop_ref:function(e){return void 0===a&&(a=z.c[508].exports),a.ug(e)},__wbindgen_string_new:function(e,t){return void 0===c&&(c=z.c[508].exports),c.h4(e,t)},__wbg_now_44a034aa2e1d73dd:function(e){return void 0===s&&(s=z.c[508].exports),s.OK(e)},__wbg_newnoargs_f579424187aa1717:function(e,t){return void 0===u&&(u=z.c[508].exports),u.bf(e,t)},__wbg_get_8bbb82393651dd9c:function(e,t){return void 0===f&&(f=z.c[508].exports),f.gM(e,t)},__wbg_call_89558c3e96703ca1:function(e,t){return void 0===l&&(l=z.c[508].exports),l.Z4(e,t)},__wbindgen_object_clone_ref:function(e){return void 0===p&&(p=z.c[508].exports),p.m_(e)},__wbg_new_55259b13834a484c:function(e,t){return void 0===d&&(d=z.c[508].exports),d.lQ(e,t)},__wbg_resolve_4f8f547f26b30b27:function(e){return void 0===b&&(b=z.c[508].exports),b.Yp(e)},__wbg_then_a6860c82b90816ca:function(e,t){return void 0===_&&(_=z.c[508].exports),_.wW(e,t)},__wbg_then_58a04e42527f52c6:function(e,t,n){return void 0===g&&(g=z.c[508].exports),g.YI(e,t,n)},__wbg_self_e23d74ae45fb17d1:function(){return void 0===h&&(h=z.c[508].exports),h.tL()},__wbg_window_b4be7f48b24ac56e:function(){return void 0===w&&(w=z.c[508].exports),w.Qu()},__wbg_globalThis_d61b1f48a57191ae:function(){return void 0===v&&(v=z.c[508].exports),v.EB()},__wbg_global_e7669da72fd7f239:function(){return void 0===m&&(m=z.c[508].exports),m.Yc()},__wbindgen_is_undefined:function(e){return void 0===y&&(y=z.c[508].exports),y.XP(e)},__wbindgen_debug_string:function(e,t){return void 0===x&&(x=z.c[508].exports),x.fY(e,t)},__wbindgen_throw:function(e,t){return void 0===k&&(k=z.c[508].exports),k.Or(e,t)},__wbindgen_rethrow:function(e){return void 0===S&&(S=z.c[508].exports),S.nD(e)},__wbindgen_closure_wrapper93:function(e,t,n){return void 0===j&&(j=z.c[508].exports),j.gi(e,t,n)}}}}},O={235:[716]},z.w={},z.f.wasm=function(e,t){(O[e]||[]).forEach((function(n,r){var o=E[n];if(o)t.push(o);else{var i,a=P[n](),c=fetch(z.p+""+{235:{716:"6f91d0681c7d7034eb5c"}}[e][n]+".module.wasm");i=a instanceof Promise&&"function"==typeof WebAssembly.compileStreaming?Promise.all([WebAssembly.compileStreaming(c),a]).then((function(e){return WebAssembly.instantiate(e[0],e[1])})):"function"==typeof WebAssembly.instantiateStreaming?WebAssembly.instantiateStreaming(c,a):c.then((function(e){return e.arrayBuffer()})).then((function(e){return WebAssembly.instantiate(e,a)})),t.push(E[n]=i.then((function(e){return z.w[n]=(e.instance||e).exports})))}}))},z(534)})();