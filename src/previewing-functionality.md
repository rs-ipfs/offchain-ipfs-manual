# Previewing the functionality in a UI

If you’re looking for a quick demo of the functionality, the simplest thing to do after running the
Docker container's default command is to launch the [substrate-front-end-template] UI.

## Instructions

1. If you have [node.js] and [yarn] installed on your machine, run the following commands:

    ```bash
    git clone https://github.com/substrate-developer-hub/substrate-front-end-template
    cd substrate-front-end-template
    yarn install
    yarn start
    ```

2. Once the UI opens in your browser, scroll down to the **Pallet Interactor** section at the bottom.
3. Keep the default "Extrinsic" active, then select `templateModule` from the first dropdown.

   <center><img alt="" src="./img/pallet-interactor-1.png" /></center>

4. Then, select the callable you want from the list of callables that become available:

   <center><img alt="" src="./img/pallet-interactor-2.png" /></center>

5. An additional text field or fields will appear below the last select box. Type the arguments
in and then click *Signed*.

   <center><img alt="" src="./img/pallet-interactor-3.png" /></center>

6. Watch your node logs and also the extrinsic events to the right for
output and information.

## Now what

This demo is based on our included `templateModule` pallet - mostly meant as a showcase
of the embedded [Rust IPFS] node. In the next section we will walk you through this pallet,
which will be instructive as a reference implementation.

[node.js]: https://nodejs.org
[yarn]: https://yarnpkg.com
[substrate-front-end-template]: https://github.com/substrate-developer-hub/substrate-front-end-template
