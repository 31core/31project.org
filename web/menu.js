fetch('/menu.html').then(res => res.text())
.then(text => {
    let body = document.getElementsByTagName('body')[0];
    let template = Handlebars.compile(body.innerHTML);
    body.innerHTML = template({
        menu: text
    });
});
