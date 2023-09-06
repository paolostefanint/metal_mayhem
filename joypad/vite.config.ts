import { defineConfig } from "vite";
import solidPlugin from "vite-plugin-solid";
import { VitePWA } from "vite-plugin-pwa";

export default defineConfig({
    base: "/j/",
    plugins: [
        solidPlugin(),
        VitePWA({
            registerType: "autoUpdate",
            includeAssets: [
                "favicon.svg",
                "favicon.ico",
                "robots.txt",
                "apple-touch-icon.png",
            ],

            manifest: {
                name: "Metal Mayhem",
                short_name: "Metal Mayhem",
                description: "Metal Mayhem Joypad app",
                theme_color: "#000000",
                icons: [
                    {
                        src: "/j/images/pwa/android-launchericon-48-48.png",
                        sizes: "48x48",
                        type: "image/png",
                    },
                    {
                        src: "/j/images/pwa/android-launchericon-72-72.png",
                        sizes: "72x72",
                        type: "image/png",
                    },
                    {
                        src: "/j/images/pwa/android-launchericon-96-96.png",
                        sizes: "96x96",
                        type: "image/png",
                    },
                    {
                        src: "/j/images/pwa/android-launchericon-144-144.png",
                        sizes: "144x144",
                        type: "image/png",
                    },
                    {
                        src: "/j/images/pwa/android-launchericon-192-192.png",
                        sizes: "192x192",
                        type: "image/png",
                    },
                    {
                        src: "/j/images/pwa/android-launchericon-512-512.png",
                        sizes: "512x512",
                        type: "image/png",
                    },
                ],
            },
        }),
    ],
    server: {
        port: 3000,
    },
    build: {
        target: "esnext",
    },
});
