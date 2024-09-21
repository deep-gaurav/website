import createGlobe from 'https://cdn.skypack.dev/cobe'

let phi = -2.80
let canvas = document.getElementById("cobe")

let width = canvas.offsetWidth;
const globe = createGlobe(canvas, {
    devicePixelRatio: 2,
    width: width * 2,
    height: width * 2,
    phi: 0,
    theta: 0.3,
    dark: 1,
    diffuse: 3,
    mapSamples: 16000,
    mapBrightness: 1.2,
    baseColor: [1, 1, 1],
    markerColor: [115 / 255, 216 / 255, 196 / 255],
    glowColor: [1.2, 1.2, 1.2],
    markers: [
        { location: [28.5355, 77.3910], size: 0.1 },
    ],
    onRender: (state) => {
        state.phi = phi
    },
})

let pointerInteracting;

canvas.onpointerdown = (e) => {
    pointerInteracting =
        e.clientX;
    canvas.style.cursor = 'grabbing';
}
canvas.onpointerup = () => {
    pointerInteracting = null;
    canvas.style.cursor = 'grab';
}
canvas.onmousemove = (e) => {
    if (pointerInteracting) {
        const delta = e.clientX - pointerInteracting;
        pointerInteracting = e.clientX
        phi += delta / 200
    }
}
canvas.ontouchstart = (e) => {
    pointerInteracting =
        e.touches[0].clientX;
}

canvas.ontouchend = (e) => {
    pointerInteracting =
        null;
}


canvas.ontouchmove = (e) => {
    if (pointerInteracting && e.touches[0]) {
        const delta = e.touches[0].clientX - pointerInteracting;
        pointerInteracting = e.touches[0].clientX
        phi += delta / 100
    }
}