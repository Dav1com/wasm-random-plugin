#let _plugin = plugin("./target/wasm32-unknown-unknown/release/wasm_random_plugin.wasm")

= abc
= hola

#{
    let x = _plugin.hello(bytes((1,)))
    linebreak()
    [#for i in range(8) [ #x.at(i)] ]
    let y = _plugin.hello(bytes((2,)))
    linebreak()
    [#for i in range(8) [ #y.at(i)] ]
}

#locate(loc => {
    let counter = counter(heading).final(loc)
    [#counter]
    let x = _plugin.hello(bytes((1,)))
    linebreak()
    [#for i in range(8) [ #x.at(i)] ]
    let y = _plugin.hello(bytes((2,)))
    linebreak()
    [#for i in range(8) [ #y.at(i)] ]
    let z = _plugin.hello(bytes((3,)))
    linebreak()
    [#for i in range(8) [ #z.at(i)] ]
})

= anotherone

#locate(loc => {
    let counter = counter(heading).final(loc)
    if counter.len() >= 4 {
        heading("wuajaja")
    }
})

#locate(loc => {
    let counter = counter(heading).final(loc)
    if counter.len() >= 3 {
        heading("wuajaja")
    }
})
