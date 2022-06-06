const config = {
    binaryThresh: 0.5,
    hiddenLayers: [10, 10, 10], // array of ints for the sizes of the hidden layers in the network
    activation: 'sigmoid', // supported activation types: ['sigmoid', 'relu', 'leaky-relu', 'tanh'],
    leakyReluAlpha: 0.01, // supported for activation type 'leaky-relu'
};

const net = new brain.NeuralNetwork(config);

const data = [
    { input: { r: 0, g: 0, b: 0 }, output: [1] },
    { input: { r: 1, g: 1, b: 1 }, output: [0] },
    { input: { r: 0, g: 0, b: 0 }, output: [1] },
    { input: { r: 1, g: 1, b: 1 }, output: [0] }, { input: { r: 0.5462865837687192, g: 0.5303043496538633, b: 0.14140393219802316 }, output: [1] }, { input: { r: 0.32598542445814505, g: 0.6649766999673383, b: 0.07225989327671556 }, output: [0] }, { input: { r: 0.4857196528977692, g: 0.0475287894031422, b: 0.6551878265029021 }, output: [1] }, { input: { r: 0.19343109153190396, g: 0.751555363814, b: 0.9711440845817612 }, output: [0] }, { input: { r: 0.5498810653736368, g: 0.11233084304043661, b: 0.7325415813887155 }, output: [1] }, { input: { r: 0.9740148028926519, g: 0.47159459925863545, b: 0.5034985547070798 }, output: [0] }, { input: { r: 0.6056231722855523, g: 0.5354761569682025, b: 0.5528282670289282 }, output: [0] }, { input: { r: 0.7127934335357158, g: 0.5517726795503881, b: 0.3962768755357262 }, output: [0] }, { input: { r: 0.6412097569932007, g: 0.4124286960590804, b: 0.5867809483418094 }, output: [0] }, { input: { r: 0.8976534728122525, g: 0.07843344283145037, b: 0.057763099236353144 }, output: [1] }, { input: { r: 0.42085426364916656, g: 0.21711889788591843, b: 0.8315702121652129 }, output: [1] }, { input: { r: 0.13949295913594795, g: 0.7055436688442547, b: 0.19346495283740195 }, output: [1] }, { input: { r: 0.8770822759549723, g: 0.2359660777268635, b: 0.3601934317666633 }, output: [0] }, { input: { r: 0.21621064423426817, g: 0.7061925975814038, b: 0.07185799719733987 }, output: [1] }, { input: { r: 0.1713652077351413, g: 0.49059263066141323, b: 0.8291930565832903 }, output: [0] }, { input: { r: 0.07477161053778136, g: 0.9193459955202286, b: 0.6102872361767444 }, output: [1] }, { input: { r: 0.571612736358043, g: 0.19869230468223886, b: 0.03350456436573479 }, output: [1] }, { input: { r: 0.24359023761154064, g: 0.1370859247693097, b: 0.8977810109918882 }, output: [1] }, { input: { r: 0.11352692351752003, g: 0.14494405587823844, b: 0.6810442909261607 }, output: [1] }, { input: { r: 0.2770507101366242, g: 0.4650836734725612, b: 0.18476547239629637 }, output: [1] }, { input: { r: 0.7429886937459207, g: 0.2030175488925332, b: 0.9099138254463146 }, output: [0] }, { input: { r: 0.4101459625959998, g: 0.5794286822683687, b: 0.9169094665281325 }, output: [0] }, { input: { r: 0.9058597800498551, g: 0.5954846838382126, b: 0.6820347090509693 }, output: [0] }, { input: { r: 0.40382138711118953, g: 0.30076772208574387, b: 0.7075336718684417 }, output: [0] }, { input: { r: 0.2343885627914195, g: 0.5103035026750875, b: 0.8309599655225826 }, output: [0] }, { input: { r: 0.4257446801770721, g: 0.565232534411001, b: 0.9050250183232156 }, output: [0] }, { input: { r: 0.25780513458280807, g: 0.805799294308057, b: 0.32033453775226595 }, output: [0] }, { input: { r: 0.8813099254992212, g: 0.9954982163696269, b: 0.16922583621467568 }, output: [0] }, { input: { r: 0.5662035073478839, g: 0.11499496121682529, b: 0.6479104945315819 }, output: [1] }, { input: { r: 0.6982438612857993, g: 0.9771525772444691, b: 0.9858296759080567 }, output: [0] }, { input: { r: 0.9931139909991362, g: 0.07586456138278441, b: 0.8231971561509752 }, output: [1] }, { input: { r: 0.35399050791181486, g: 0.3674331708795, b: 0.05472302928195094 }, output: [1] }
];

net.train(data);

// const $diagram = document.getElementById("diagram");
// $diagram.innerHTML = brain.utilities.toSVG(net);

const $colorBox = document.getElementById("color");
const $buttonWhite = document.getElementById("button-white");
const $buttonBlack = document.getElementById("button-black");
const $buttonGuess = document.getElementById("button-guess");
const $guessedText = document.getElementById("guess");
const $buttonRefresh = document.getElementById("button-refresh");

main();

function main() {
    let randomColor = getRandomColor();
    setRandomColorForColorBox(randomColor);
    setGuessColor(randomColor);
    $buttonBlack.addEventListener('click', () => {
        console.log('black');
        data.push({
            input: randomColor,
            output: [0],
        });
        randomColor = getRandomColor();
        setRandomColorForColorBox(randomColor);
        setGuessColor(randomColor);
    });
    $buttonWhite.addEventListener('click', () => {
        console.log('white');
        data.push({
            input: randomColor,
            output: [1],
        });
        randomColor = getRandomColor();
        setRandomColorForColorBox(randomColor);
        setGuessColor(randomColor);
    });
    $buttonGuess.addEventListener('click', () => {
        console.log(JSON.stringify(data));
        net.train(data);
    })
    $buttonRefresh.addEventListener('click', () => {
        randomColor = getRandomColor();
        setRandomColorForColorBox(randomColor);
        setGuessColor(randomColor);
    });
}

function getRandomColor() {
    return {
        r: Math.random(),
        g: Math.random(),
        b: Math.random(),
    }
}

function setGuessColor(color) {
    const guess = net.run(color)[0];
    console.log(`guess: ${guess}`);
    const colorValue = getBgColor();
    $guessedText.style.color = colorValue;

    function getBgColor() {
        if (guess < 0.5) {
            return '#000';
        } else {
            return '#fff';
        }
    }
}

function setRandomColorForColorBox(color) {
    $colorBox.style.backgroundColor =
        `rgba(${color.r * 255}, ${color.g * 255}, ${color.b * 255}, 1)`;
}