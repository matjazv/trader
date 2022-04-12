const metamaskStatus = document.querySelector('.metamaskStatus');

if (typeof window.ethereum !== 'undefined') {
    metamaskStatus.innerHTML = 'MetaMask is installed!';
} else {
    metamaskStatus.innerHTML = 'MetaMask is NOT installed!';
}

const loginButton = document.querySelector('.loginWithMetamaskButton');
const logoutButton = document.querySelector('.logoutButton');
const showAccountButton = document.querySelector('.showAccountButton');
const showAccount = document.querySelector('.showAccount');
const signMessageButton = document.querySelector('.signMessageButton');
const showSignedMessage = document.querySelector('.showSignedMessage');
const showCheckSignedMessage = document.querySelector('.showCheckSignedMessage');

if (loginButton !== null) {
    loginButton.addEventListener('click', () => {
        loginUser();
    });
}

if (logoutButton !== null) {
    logoutButton.addEventListener('click', () => {
        logoutUser();
    });
}

if (showAccountButton !== null) {
    showAccountButton.addEventListener('click', () => {
        getAccount();
    });
}

if (signMessageButton !== null) {
    signMessageButton.addEventListener('click', () => {
        signMessage();
    });
}

async function getAccount() {
    const accounts = await ethereum.request({method: 'eth_requestAccounts'});
    const account = accounts[0];
    showAccount.innerHTML = account;
}

function logoutUserListener () {
    window.location.replace("/");
}

async function logoutUser() {
    var oReq = new XMLHttpRequest();
    oReq.addEventListener("load", logoutUserListener);
    oReq.open("GET", "/logout");
    oReq.send();
}

async function loginUserListener(evt) {
    location.reload();
}

async function loginUser() {
    // A Web3Provider wraps a standard Web3 provider, which is
    // what MetaMask injects as window.ethereum into each page
    const provider = new ethers.providers.Web3Provider(window.ethereum)

    // MetaMask requires requesting permission to connect users accounts
    await provider.send("eth_requestAccounts", []);

    // The MetaMask plugin also allows signing transactions to
    // send ether and pay to change state within the blockchain.
    // For this, you need the account signer...
    const signer = await provider.getSigner()

    let message = "Hi there from IOTA Trader! Sign this message to prove you have access to this wallet and we'll log you in. This won't cost you any IOTA.\n" +
        "To stop hackers using your wallet, here's a unique message ID they can't guess: d458fa15-dcab-4d85-a477–004d6febca12"

    let signature = await signer.signMessage(message);

    if (showSignedMessage !== null) {
        showSignedMessage.innerHTML = signature;
    }

    const accounts = await ethereum.request({method: 'eth_requestAccounts'});
    const account = accounts[0];

    var xhr = new XMLHttpRequest();
    xhr.addEventListener("load", loginUserListener);
    xhr.open("POST", "/login", true);
    xhr.setRequestHeader('Content-Type', 'application/json');
    xhr.send(JSON.stringify({
        account: account.toString(),
        signature: signature,
    }));
}

async function signMessage() {
    // A Web3Provider wraps a standard Web3 provider, which is
    // what MetaMask injects as window.ethereum into each page
    const provider = new ethers.providers.Web3Provider(window.ethereum)

    // MetaMask requires requesting permission to connect users accounts
    await provider.send("eth_requestAccounts", []);

    // The MetaMask plugin also allows signing transactions to
    // send ether and pay to change state within the blockchain.
    // For this, you need the account signer...
    const signer = await provider.getSigner()

    let message = "Hi there from IOTA Trader! Sign this message to prove you have access to this wallet and we'll log you in. This won't cost you any IOTA.\n" +
        "To stop hackers using your wallet, here's a unique message ID they can't guess: d458fa15-dcab-4d85-a477–004d6febca12"

    let signature = await signer.signMessage(message);

    if (showSignedMessage !== null) {
        showSignedMessage.innerHTML = signature;
    }

    // Check signature
    let address = ethers.utils.verifyMessage(message, signature);
    if (showCheckSignedMessage !== null) {
        showCheckSignedMessage.innerHTML = address;
    }
}
