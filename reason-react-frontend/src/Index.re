[@bs.val] external document: Js.t({..}) = "document";
[@bs.val] external requireCSS: string => unit = "require";

let h1Style = ReactDOMRe.Style.make(~textAlign="center", ());
let contentStyle =
  ReactDOMRe.Style.make(~color="#444444", ~width="60%", ~margin="auto", ());

ReactDOMRe.renderToElementWithId(
  <>
    <h1 style=h1Style> {React.string("Hello")} </h1>
    <div style=contentStyle> <NumberInput /> </div>
  </>,
  "root",
);