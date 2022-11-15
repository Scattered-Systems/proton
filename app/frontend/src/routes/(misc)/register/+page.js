import { info } from '$lib/constants.js';

/** @type {import('./$types').PageLoad} */
export async function load({ locals }) {
    return {
        title: 'Register',
        components: [
            {
                id: 0,
                key: 'header',
                data: [

                ]
            }
        ],
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