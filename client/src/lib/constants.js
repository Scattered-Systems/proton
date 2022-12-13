
export default {
    
    
    origin: "pzzld.eth",
    theme: {
        colors: {
            primary: "",
            secondary: ""
        }
    }
}

export let info = {
    homepage: "/",
    metadata: {
        github: "scattered-systems",
        twitter: "scsys_eth"
    },
    name: "Proton",
    slug: "proton",
    sitemap: {
        href: "/",
        label: "Home",
        data: [
            {
                layout: "dashboard",
                label: "Dashboard",
                data: []
            }
        ]
    },
}

export let colors = {
    gradients: {
        cyan: ["bg-gradient-to-r from-cyan-700 via-cyan-500 to-cyan-900 text-white"]
    }
}

export let theme = {
    secondary: colors.gradients.cyan[0],
    button: {
        primary: "flex flex-initial px-3 py-1 rounded items-center justify-center"
    },
    link: "flex flex-initial overflow-y-clip hover:underline px-3 py-2 hover:opacity-75 text-white"
}

export let metadata = {

    homepage: '/',
    name: 'Puzzled',
    slug: 'pzzld',
    tags: [],
    url: 'https://app.pzzld.eth.limo',
    data: [
        {
            id: 0,
            label: 'github',
            data: ['FL03']
        },
        {
            id: 1,
            label: 'linkedin',
            data: ['scsys_eth']
        },
        {
            id: 2,
            label: 'twitter',
            data: ['jo3mccain']
        }
    ]
}

