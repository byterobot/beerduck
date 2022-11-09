import Asciidoctor from 'https://deno.land/x/asciidoctor/mod.js';

// let html = "";
// try {
//     let text = Deno.core.ops.op_adoc();
//     let options = { safe: "server", doctype: "article", standalone: true, attributes: "linkcss", };
//     html = Asciidoctor().convert(text, options,);
// } catch (e) {
//     Deno.core.print("Exception: " + e, true);
// }
// html

let text = Deno.core.ops.op_adoc();
let options = { safe: "server", doctype: "article", standalone: true, attributes: "linkcss", };
Asciidoctor().convert(text, options);
