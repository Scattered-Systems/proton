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
                href: "/dashboard",
                label: "Dashboard",
                data: [
                    {
                        href: "/dashboard/account",
                        label: "Dashboard"
                    },
                    {
                        href: "/dashboard/community",
                        label: "Dashboard"
                    },
                    {
                        href: "/dashboard/content",
                        label: "Dashboard"
                    },
                    {
                        href: "/dashboard/discover",
                        label: "Dashboard"
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
