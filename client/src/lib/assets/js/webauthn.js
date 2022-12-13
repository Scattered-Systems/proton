// To abort a WebAuthn call, instantiate an `AbortController`.
const abortController = new AbortController();

const publicKeyCredentialRequestOptions = (challenge) => {
    return {
        // Server generated challenge
        challenge: challenge,
        // The same RP ID as used during registration
        rpId: 'example.com',
    }
  
};

const credential = await navigator.credentials.get({
  publicKey: publicKeyCredentialRequestOptions,
  signal: abortController.signal,
  // Specify 'conditional' to activate conditional UI
  mediation: 'conditional'
});