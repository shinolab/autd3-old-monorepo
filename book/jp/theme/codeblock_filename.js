document.addEventListener('DOMContentLoaded', () => {
    const code = document.getElementsByTagName('code');
    Array.from(code).forEach(el => {
        if (el.className) {
            const tokens = el.className.split(' ');
            const filenameClass = tokens.find(c => c.startsWith('filename='));
            const setHeader = (el, header) => {
                el.parentElement.setAttribute('data-lang', header);
                el.parentElement.classList.add('code-block-header');
            };
            if (filenameClass) {
                const filename = filenameClass.replace('filename=', '');
                if (filename.length > 0)
                    setHeader(el, filename);
            } else {
                const header = tokens[0];
                if (header.startsWith('language-')) {
                    const progNameRaw = header.replace('language-', '');
                    let progName = progNameRaw.charAt(0).toUpperCase() + progNameRaw.slice(1);
                    if (progName === 'Cpp') progName = 'C++';
                    if (progName === 'Cs') progName = 'C#';
                    setHeader(el, progName);
                }
            }
        }
    });
});
