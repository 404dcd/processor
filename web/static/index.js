// Register a new language
monaco.languages.register({ id: 'qsis16-lang' });

// Register a tokens provider for the language
monaco.languages.setMonarchTokensProvider('qsis16-lang', {
    // Set defaultToken to invalid to see what you do not tokenize yet
    // defaultToken: 'invalid',

    instrs: [
        'add', 'mul', 'mulh', 'div', 'mod', 'addi', 'subi', 'shl', 'shr', 'rol', 'ror',
        'not', 'neg', 'imm', 'out', 'or', 'xor', 'and', 'nop', 'mov', 'ld', 'sto'
    ],

    pcmods: [
        'beq', 'blt', 'jmp', 'hlt'
    ],

    regs: [
        '$a', '$b', '$c', '$d', '$e', '$f', '$g', '$h', '$i', '$j',
        '$k', '$l', '$m', '$0', '$pc'
    ],

    // The main tokenizer for our languages
    tokenizer: {
        root: [
            // keywords
            [/[a-z_$][\w$]*/, {
                cases: {
                    '@instrs': 'instr',
                    '@pcmods': 'pcmod',
                    '@regs': 'reg'
                }
            }],

            [/(0b|0x)?\d+/, 'number'],
            [/;.*/, 'comment'],
            [/.\w+:/, 'label']

        ],
    },
});

// Define a new theme that contains only rules that match this language
monaco.editor.defineTheme('qsis16-theme-light', {
    base: 'vs',
    inherit: true,
    rules: [
        { token: 'instr', foreground: '0000ff' },
        { token: 'pcmod', foreground: '0000ff', fontStyle: 'bold' },
        { token: 'reg', foreground: 'd00000' },
        { token: 'number', foreground: '00a000' },
        { token: 'comment', foreground: 'a0a0a0', fontStyle: 'italic' },
        { token: 'label', foreground: '000000', fontStyle: 'bold' },
    ]
});

monaco.editor.defineTheme('qsis16-theme-dark', {
    base: 'vs-dark',
    inherit: true,
    rules: [
        { token: 'instr', foreground: '4edde6' },
        { token: 'pcmod', foreground: '5cf6ff', fontStyle: 'bold' },
        { token: 'reg', foreground: 'c34069' },
        { token: 'number', foreground: 'b796ff' },
        { token: 'comment', foreground: '828277', fontStyle: 'italic' },
        { token: 'label', foreground: 'e0e0e0', fontStyle: 'bold' },
    ]
});

window.editor = monaco.editor.create(document.getElementById("editor"), {
    theme: 'qsis16-theme-light',
    value: `    imm 2 $a
    imm 97 $k
    out $a
    addi 1 $a
    out $a
.newprm: ; outer loop
    addi 2 $a
    imm 3 $i
    mov $a $f
    shr 1 $f
.check: ; inner loop
    mod $a $i $b
    beq $b $0 newprm ; this mod was 0
    addi 2 $i
    blt $i $f check

    out $a
    blt $a $k newprm
    hlt`,
    language: 'qsis16-lang'
});


async function runCode() {
    const res = await fetch("/", {
        method: 'POST',
        headers: {
            'Content-Type': 'application/x-www-form-urlencoded',
        },
        body: window.editor.getValue()
    })
    const text = await res.text();
    document.getElementById('output').innerText = text;
};

function saveCode() {
    const a = document.createElement('a');
    const file = new Blob([window.editor.getValue()], { type: 'text/plain' });

    a.href = URL.createObjectURL(file);
    a.download = 'code.asm';
    a.click();

    URL.revokeObjectURL(a.href);
};

function loadCode() {
    const inp = document.createElement('input');
    inp.setAttribute('type', 'file');
    inp.onchange = async () => {
        const text = await inp.files[0].text()
        window.editor.setValue(text)
    }
    inp.click()
}

function toggleTheme() {
    const toggle = document.getElementById('themeTog');
    if (toggle.innerText === "🌙") {
        monaco.editor.setTheme('qsis16-theme-dark');
        toggle.innerText = "☀️"
        document.getElementById('output').style = "background:#303030; color:#ffffff";
        document.getElementById('buttonBar').style = "background:#303030";
    } else {
        monaco.editor.setTheme('qsis16-theme-light');
        toggle.innerText = "🌙";
        document.getElementById('output').style = "";
        document.getElementById('buttonBar').style = "";
    }
}