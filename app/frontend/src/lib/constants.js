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
                data: [
                    {
                        href: "/account",
                        label: "Account"
                    },
                    {
                        href: "/community",
                        label: "Community"
                    },
                    {
                        href: "/content",
                        label: "Content"
                    },
                    {
                        href: "/discover",
                        label: "Discover"
                    }
                ]
            }
        ]
    },
    theme: {
        button: {
            layout: {
                primary: 'flex flex-auto mx-auto px-3 py-1 rounded items-center justify-center',
                round: 'flex flex-auto mx-auto px-3 py-1 rounded-full items-center justify-center'
            },
        },
        color: {
            gradient: { 
                primary: 'bg-gradient-to-r from-cyan-700 via-cyan-500 to-cyan-900 text-white'
            }
            ,
            solid: {
                primary: ''
            }
        },
        form: {
            layout: [
                'flex rounded-full px-3 text-black'
            ]
        }
    }
}

export let colors = {
    gradients: {
        cyan: ["bg-gradient-to-r from-cyan-700 via-cyan-500 to-cyan-900 text-white"]
    }
}

export let theme = {
    button: {
        primary: "flex flex-initial mx-auto px-3 py-1 rounded items-center justify-center" + colors.gradients.cyan[0]
    },
    link: "flex flex-initial overflow-y-clip hover:underline px-3 py-2 hover:opacity-75 text-white"
}