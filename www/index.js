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
                this.content.textContent = data.new_color;

                m.draw(data.new_color);
            })
        })
        .catch(console.error);
    }
};

const changeColorButton = document.getElementById("change-color");

changeColorButton.addEventListener("click", event => {
  console.log(event);
  content.render();
});

setInterval(function() { 
    content.render();
}, 1000);
