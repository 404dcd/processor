// Register a new language
monaco.languages.register({ id: "qsis16-lang" });

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

            [/\d+/, 'number'],
            [/;.*/, 'comment'],
            [/.\w+:/, 'label']

        ],
    },
});

// Define a new theme that contains only rules that match this language
monaco.editor.defineTheme('qsis16-theme', {
    base: 'vs',
    inherit: false,
    rules: [
        { token: 'instr', foreground: '0000ff' },
        { token: 'pcmod', foreground: '0000ff', fontStyle: 'bold' },
        { token: 'reg', foreground: 'd00000' },
        { token: 'number', foreground: '00a000' },
        { token: 'comment', foreground: 'a0a0a0', fontStyle: 'italic' },
        { token: 'label', foreground: '000000', fontStyle: 'bold' },
    ]
});

window.editor = monaco.editor.create(document.getElementById("editor"), {
    theme: 'qsis16-theme',
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
    document.getElementById("output").innerText = text;
};

function saveCode() {
    const a = document.createElement('a');
    const file = new Blob([window.editor.getValue()], { type: 'text/plain' });

    a.href = URL.createObjectURL(file);
    a.download = 'code.asm';
    a.click();

    URL.revokeObjectURL(a.href);
};