const metamaskStatus = document.querySelector('.metamaskStatus');

if (typeof window.ethereum !== 'undefined') {
    metamaskStatus.innerHTML = 'MetaMask is installed!';
} else {
    metamaskStatus.innerHTML = 'MetaMask is NOT installed!';
}

const loginButton = document.querySelector('.loginWithMetamaskButton');
const showAccount = document.querySelector('.showAccount');

loginButton.addEventListener('click', () => {
    getAccount();
});

async function getAccount() {
    const accounts = await ethereum.request({method: 'eth_requestAccounts'});
    const account = accounts[0];
    showAccount.innerHTML = account;
}

const signMessageButton = document.querySelector('.signMessageButton');
const showSignedMessage = document.querySelector('.showSignedMessage');

signMessageButton.addEventListener('click', () => {
    signMessage();
});

async function signMessage() {
    // A Web3Provider wraps a standard Web3 provider, which is
    // what MetaMask injects as window.ethereum into each page
    const provider = new ethers.providers.Web3Provider(window.ethereum)
    ethereum.sign
    // MetaMask requires requesting permission to connect users accounts
    await provider.send("eth_requestAccounts", []);

    // The MetaMask plugin also allows signing transactions to
    // send ether and pay to change state within the blockchain.
    // For this, you need the account signer...
    const signer = provider.getSigner()

    let signature = await signer.signMessage("Hi there from IOTA Trader! Sign this message to prove you have access to this wallet and we'll log you in. This won't cost you any IOTA.\n" +
        "To stop hackers using your wallet, here's a unique message ID they can't guess: d458fa15-dcab-4d85-a477–004d6febca12");

    showSignedMessage.innerHTML = signature;

    var xhr = new XMLHttpRequest();
    xhr.open("POST", "/login", true);
    xhr.setRequestHeader('Content-Type', 'application/json');
    xhr.send(JSON.stringify({
        account: showAccount.innerHTML.toString(),
        signature: signature,
    }));
}
