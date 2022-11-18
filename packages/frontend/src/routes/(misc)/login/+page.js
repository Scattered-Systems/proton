/** @type {import('./$types').PageLoad} */
export async function load({ locals }) {
    return {
        title: 'Login',
        ctrl: {
            submit: { label: "Submit" },
            wallet: { label: "Wallet" }
        },
        form: [
            {
                id: 'loginForm',
                data: [
                    {
                        id: 'inputFormUsername',
                        data: "",
                        type: 'string'
                    },
                    {
                        id: 'inputFormPassword',
                        data: '',
                        type: 'password'
                    }
                ]
            }
        ]
    };
};