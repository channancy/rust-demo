const rust = import('../pkg');

const content = new class {
  constructor() {
        this.content = document.getElementById("content");
    }

    render() {
        rust
        .then(m => {
            return m.run("channancy/rust-demo").then((data) => {
                console.log(data);
                this.content.textContent = data.commit.commit.author.email;

                console.log("The latest commit to the wasm-bindgen %s branch is:", data.name);
                console.log("%s, authored by %s <%s>", data.commit.sha, data.commit.commit.author.name, data.commit.commit.author.email);
            })
        })
        .catch(console.error);
    }
};

content.render();