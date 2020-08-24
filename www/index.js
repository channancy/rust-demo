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

                m.draw();
            })
        })
        .catch(console.error);
    }
};

content.render();
