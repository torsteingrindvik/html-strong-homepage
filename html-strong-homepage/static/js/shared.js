document.addEventListener("DOMContentLoaded", function() {
    hljs.highlightAll();
    document.querySelectorAll('code.rust-inline').forEach(el => {
        hljs.highlightElement(el);
    });
});
