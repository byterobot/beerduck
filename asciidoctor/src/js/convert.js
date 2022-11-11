import Asciidoctor from 'https://deno.land/x/asciidoctor/mod.js';

let text = Deno.core.ops.op_adoc();
let options = { safe: "server", doctype: "article", standalone: true, attributes: "linkcss", };
Asciidoctor().convert(text, options);
