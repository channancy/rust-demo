const rust = import('../pkg');

const content = new class {
    constructor() {}

    render() {
        rust
        .then(m => {
            return m.get_colors().then((data) => {
                let colors = data.colors;

                colors.forEach((element, index) => {
                    m.draw(element.hex, index);
                });
            })
        })
        .catch(console.error);
    }
};

setInterval(() => content.render(), 2000);
