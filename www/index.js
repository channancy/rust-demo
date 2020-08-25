const rust = import('../pkg');

const content = new class {
  constructor() {
        this.content = document.getElementById("content");
    }

    render() {
        rust
        .then(m => {
            return m.run().then((data) => {
                console.log(data);
                let colors = data.colors;

                m.draw(colors[0].hex, "1");
                m.draw(colors[1].hex, "2");
            })
        })
        .catch(console.error);
    }
};

setInterval(function() { 
    content.render();
}, 2000);
