const rust = import('../pkg');

const content = new class {
    constructor() {}

    render() {
        rust
        .then(m => {
            m.draw(1);
        })
        .catch(console.error);
    }
};

setInterval(() => content.render(), 2000);
