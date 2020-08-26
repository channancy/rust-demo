const rust = import('../pkg');

const content = new class {
    constructor() {
        this.elements = ['canvas1', 'canvas2', 'canvas3', 'canvas4', 'canvas5', 'canvas6'];
    }

    render() {
        rust
        .then(m => {
            this.elements.forEach(element => {
                m.draw(element);
            });
        })
        .catch(console.error);
    }
};

setInterval(() => content.render(), 2000);
