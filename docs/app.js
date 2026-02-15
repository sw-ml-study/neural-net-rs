// Neural Network Training Platform - Application Logic
// Minimal JavaScript for bootstrapping WASM and DOM manipulation
// All neural network logic implemented in Rust/WASM

import init, {
    NeuralNetwork,
    listExamples,
    getExampleInfo,
    getExampleData
} from './wasm/neural_net_wasm.js?ts=1771160891000';

// Application state
let wasmModule;
let currentNetwork = null;
let currentExampleInfo = null;
let trainingData = null;
let lossHistory = [];
let trainingStartTime = null;
let trainingInterval = null;
let eventSource = null;

// Educational content for each example
const exampleEducation = {
    and: {
        title: "AND Gate - Linearly Separable Problem",
        content: `
            <h3>What is an AND Gate?</h3>
            <p>The AND gate is a fundamental logic gate that outputs 1 (true) only when <strong>both</strong> inputs are 1. In all other cases, it outputs 0 (false).</p>

            <table class="truth-table-mini">
                <thead><tr><th>A</th><th>B</th><th>A AND B</th></tr></thead>
                <tbody>
                    <tr><td>0</td><td>0</td><td>0</td></tr>
                    <tr><td>0</td><td>1</td><td>0</td></tr>
                    <tr><td>1</td><td>0</td><td>0</td></tr>
                    <tr><td>1</td><td>1</td><td>1</td></tr>
                </tbody>
            </table>

            <h3>Why is it "Linearly Separable"?</h3>
            <p>A problem is <strong>linearly separable</strong> when you can draw a single straight line to separate the two classes (0s and 1s). For AND, you can draw a line that puts (1,1) on one side and all other points on the other.</p>

            <div class="highlight">
                <strong>Key Insight:</strong> Linearly separable problems can be solved by a single-layer perceptron (no hidden layers needed). However, we use a hidden layer here for consistency and to demonstrate the architecture.
            </div>

            <h3>Network Architecture</h3>
            <p>This demo uses a <span class="network-type">Feed-Forward Neural Network</span> with architecture [2, 2, 1]:</p>
            <ul>
                <li><strong>2 input neurons</strong> - one for each input bit</li>
                <li><strong>2 hidden neurons</strong> - learn feature combinations</li>
                <li><strong>1 output neuron</strong> - produces the AND result</li>
            </ul>
        `
    },
    or: {
        title: "OR Gate - Linearly Separable Problem",
        content: `
            <h3>What is an OR Gate?</h3>
            <p>The OR gate outputs 1 (true) when <strong>at least one</strong> input is 1. It only outputs 0 when both inputs are 0.</p>

            <table class="truth-table-mini">
                <thead><tr><th>A</th><th>B</th><th>A OR B</th></tr></thead>
                <tbody>
                    <tr><td>0</td><td>0</td><td>0</td></tr>
                    <tr><td>0</td><td>1</td><td>1</td></tr>
                    <tr><td>1</td><td>0</td><td>1</td></tr>
                    <tr><td>1</td><td>1</td><td>1</td></tr>
                </tbody>
            </table>

            <h3>Linearly Separable</h3>
            <p>Like AND, OR is linearly separable. You can draw a single line separating (0,0) from all other points. A single-layer perceptron could learn this.</p>

            <h3>Comparison with AND</h3>
            <p>Notice the difference: AND has one "1" output, while OR has three. The network learns different weight patterns to distinguish these cases.</p>

            <h3>Network Architecture</h3>
            <p><span class="network-type">Feed-Forward Neural Network</span> [2, 2, 1] - same as AND, but learns different weights.</p>
        `
    },
    xor: {
        title: "XOR Gate - The Classic Non-Linear Problem",
        content: `
            <h3>What is XOR?</h3>
            <p>XOR (exclusive OR) outputs 1 when inputs are <strong>different</strong>, and 0 when they're the same.</p>

            <table class="truth-table-mini">
                <thead><tr><th>A</th><th>B</th><th>A XOR B</th></tr></thead>
                <tbody>
                    <tr><td>0</td><td>0</td><td>0</td></tr>
                    <tr><td>0</td><td>1</td><td>1</td></tr>
                    <tr><td>1</td><td>0</td><td>1</td></tr>
                    <tr><td>1</td><td>1</td><td>0</td></tr>
                </tbody>
            </table>

            <h3>Why is XOR Important?</h3>
            <div class="highlight">
                <strong>Historical Significance:</strong> In 1969, Minsky and Papert showed that single-layer perceptrons cannot learn XOR. This "AI Winter" contributed to reduced neural network research for years. The solution: <strong>multi-layer networks with hidden layers</strong>.
            </div>

            <h3>Not Linearly Separable</h3>
            <p>You cannot draw a single straight line to separate the 1s (at corners 01 and 10) from the 0s (at corners 00 and 11). This requires a hidden layer to create non-linear decision boundaries.</p>

            <h3>How the Network Solves It</h3>
            <p>The hidden layer learns to decompose XOR into simpler operations:</p>
            <ul>
                <li>One hidden neuron might learn "A OR B"</li>
                <li>Another might learn "NOT (A AND B)"</li>
                <li>The output combines them: (A OR B) AND NOT(A AND B) = XOR</li>
            </ul>

            <h3>Network Architecture</h3>
            <p><span class="network-type">Feed-Forward Neural Network</span> [2, 3, 1] - needs 3 hidden neurons to learn the non-linear boundary.</p>
        `
    },
    parity3: {
        title: "3-Bit Parity - Extended XOR",
        content: `
            <h3>What is Parity?</h3>
            <p>Parity checks if the count of 1s is odd or even. The 3-bit parity function outputs 1 when an <strong>odd number</strong> of inputs are 1.</p>

            <table class="truth-table-mini">
                <thead><tr><th>A</th><th>B</th><th>C</th><th>Count</th><th>Output</th></tr></thead>
                <tbody>
                    <tr><td>0</td><td>0</td><td>0</td><td>0 (even)</td><td>0</td></tr>
                    <tr><td>0</td><td>0</td><td>1</td><td>1 (odd)</td><td>1</td></tr>
                    <tr><td>0</td><td>1</td><td>1</td><td>2 (even)</td><td>0</td></tr>
                    <tr><td>1</td><td>1</td><td>1</td><td>3 (odd)</td><td>1</td></tr>
                </tbody>
            </table>

            <h3>Relation to XOR</h3>
            <p>Parity is essentially XOR extended to more inputs: <code>A XOR B XOR C</code>. Like XOR, it's non-linearly separable and requires hidden layers.</p>

            <h3>Why It's Harder</h3>
            <p>With 3 inputs, there are 8 possible combinations (2³). The network must learn more complex decision boundaries than 2-input XOR.</p>

            <div class="highlight">
                <strong>Training Tip:</strong> Parity problems can get stuck in local minima depending on random weight initialization. If you see high errors after training, try again - each run starts with different random weights. Using 20000 epochs and learning rate 0.5-0.8 usually converges well.
            </div>

            <h3>Network Architecture</h3>
            <p><span class="network-type">Feed-Forward Neural Network</span> [3, 6, 1] - 6 hidden neurons provide enough capacity to reliably learn the parity function.</p>
        `
    },
    quadrant: {
        title: "Quadrant Classification - Multi-Class Output",
        content: `
            <h3>What is Quadrant Classification?</h3>
            <p>Given a 2D point (x, y), classify which quadrant of the Cartesian plane it belongs to:</p>
            <ul>
                <li><strong>Quadrant I:</strong> x > 0, y > 0 (top-right)</li>
                <li><strong>Quadrant II:</strong> x < 0, y > 0 (top-left)</li>
                <li><strong>Quadrant III:</strong> x < 0, y < 0 (bottom-left)</li>
                <li><strong>Quadrant IV:</strong> x > 0, y < 0 (bottom-right)</li>
            </ul>

            <h3>Multi-Class Classification</h3>
            <p>Unlike binary problems (AND, OR, XOR), this has <strong>4 possible outputs</strong>. We use <strong>one-hot encoding</strong>: each class is represented by activating one output neuron.</p>

            <div class="highlight">
                <strong>One-Hot Encoding:</strong> Class 1 = [1,0,0,0], Class 2 = [0,1,0,0], etc. The network outputs probabilities for each class.
            </div>

            <h3>Network Architecture</h3>
            <p><span class="network-type">Feed-Forward Neural Network</span> [2, 4, 4]:</p>
            <ul>
                <li><strong>2 inputs</strong> - x and y coordinates</li>
                <li><strong>4 hidden neurons</strong> - learn quadrant boundaries</li>
                <li><strong>4 outputs</strong> - one per quadrant (one-hot)</li>
            </ul>
        `
    },
    adder2: {
        title: "2-Bit Binary Adder - Arithmetic Learning",
        content: `
            <h3>What Does It Do?</h3>
            <p>Adds two 2-bit binary numbers (0-3 each) and outputs the 3-bit sum (0-6).</p>
            <p>Example: 2 + 3 = 5 → [1,0] + [1,1] = [1,0,1]</p>

            <h3>Why It's Impressive</h3>
            <p>The network learns binary arithmetic from examples alone - no explicit rules about carry bits or binary addition. It discovers these patterns through gradient descent.</p>

            <div class="highlight">
                <strong>Deep Learning Insight:</strong> This demonstrates that neural networks can learn abstract mathematical operations purely from input-output examples, without being programmed with the rules.
            </div>

            <h3>Input/Output Format</h3>
            <ul>
                <li><strong>Input:</strong> [A1, A0, B1, B0] - two 2-bit numbers</li>
                <li><strong>Output:</strong> [S2, S1, S0] - 3-bit sum</li>
            </ul>

            <h3>Network Architecture</h3>
            <p><span class="network-type">Feed-Forward Neural Network</span> [4, 8, 3] - 67 parameters to learn binary addition.</p>
        `
    },
    iris: {
        title: "Iris Flower Classification - Real-World Data",
        content: `
            <h3>The Famous Iris Dataset</h3>
            <p>Introduced by statistician Ronald Fisher in 1936, this is one of the most famous datasets in machine learning history. It classifies iris flowers into 3 species based on petal and sepal measurements.</p>

            <h3>Input Features (in cm)</h3>
            <ul>
                <li>Sepal length (4.3-7.9)</li>
                <li>Sepal width (2.0-4.4)</li>
                <li>Petal length (1.0-6.9)</li>
                <li>Petal width (0.1-2.5)</li>
            </ul>

            <h3>Output Classes</h3>
            <ul>
                <li><strong>Setosa</strong> - small petals, easily separable</li>
                <li><strong>Versicolor</strong> - medium petals</li>
                <li><strong>Virginica</strong> - large petals, overlaps with Versicolor</li>
            </ul>

            <div class="highlight">
                <strong>Real-World Challenge:</strong> Unlike synthetic examples, Iris has natural variation and class overlap. Versicolor and Virginica aren't perfectly separable, testing the network's generalization ability.
            </div>

            <h3>Network Architecture</h3>
            <p><span class="network-type">Feed-Forward Neural Network</span> [4, 8, 3] - same as adder2 but learns biological patterns instead of arithmetic.</p>
        `
    },
    pattern3x3: {
        title: "3x3 Pattern Recognition - Visual Learning",
        content: `
            <h3>What is Pattern Recognition?</h3>
            <p>The network learns to recognize visual patterns in a 3x3 pixel grid - a tiny version of image classification.</p>

            <h3>The Patterns</h3>
            <table class="truth-table-mini">
                <thead><tr><th>X (diagonals)</th><th>O (border)</th><th>+ (cross)</th><th>- (line)</th></tr></thead>
                <tbody><tr>
                    <td><code>1 0 1<br>0 1 0<br>1 0 1</code></td>
                    <td><code>1 1 1<br>1 0 1<br>1 1 1</code></td>
                    <td><code>0 1 0<br>1 1 1<br>0 1 0</code></td>
                    <td><code>0 0 0<br>1 1 1<br>0 0 0</code></td>
                </tr></tbody>
            </table>

            <h3>Connection to Computer Vision</h3>
            <p>This is a miniature version of how CNNs (Convolutional Neural Networks) recognize images. The hidden neurons learn to act as <strong>feature detectors</strong>:</p>
            <ul>
                <li>Diagonal detectors (for X)</li>
                <li>Border/edge detectors (for O)</li>
                <li>Cross detectors (for +)</li>
            </ul>

            <div class="highlight">
                <strong>Scaling Challenge:</strong> With 9 inputs, this is our largest example (88 parameters). Real images (e.g., 224x224 = 50,176 pixels) require much more efficient architectures like CNNs.
            </div>

            <h3>Network Architecture</h3>
            <p><span class="network-type">Feed-Forward Neural Network</span> [9, 6, 4] - 9 inputs (one per pixel), 4 outputs (one per pattern).</p>
        `
    }
};

// Get educational content for current example
function getExampleEducation(exampleName) {
    return exampleEducation[exampleName] || {
        title: "About This Example",
        content: "<p>Educational content not available for this example.</p>"
    };
}

// DOM elements
const elements = {
    exampleSelect: document.getElementById('example-select'),
    exampleDescription: document.getElementById('example-description'),
    epochsInput: document.getElementById('epochs-input'),
    learningRateInput: document.getElementById('learning-rate-input'),
    seedInput: document.getElementById('seed-input'),
    trainButton: document.getElementById('train-button'),
    stopButton: document.getElementById('stop-button'),
    evaluateButton: document.getElementById('evaluate-button'),
    trainingStatus: document.querySelector('.status-text'),
    progressContainer: document.getElementById('progress-container'),
    progressFill: document.getElementById('progress-fill'),
    progressText: document.getElementById('progress-text'),
    currentEpoch: document.getElementById('current-epoch'),
    currentLoss: document.getElementById('current-loss'),
    trainingTime: document.getElementById('training-time'),
    input1: document.getElementById('input1'),
    input2: document.getElementById('input2'),
    outputDisplay: document.getElementById('output-display'),
    architectureDisplay: document.getElementById('architecture-display'),
    truthTableBody: document.getElementById('truth-table-body'),
    lossChart: document.getElementById('loss-chart'),
};

// Chart context
const chartCtx = elements.lossChart.getContext('2d');
let chartWidth = elements.lossChart.width;
let chartHeight = elements.lossChart.height;

// Initialize application
async function initApp() {
    try {
        // Initialize WASM module
        wasmModule = await init();
        console.log('WASM module loaded successfully');

        // Load examples
        await loadExamples();

        // Setup event listeners
        setupEventListeners();

        // Update UI
        updateExampleInfo();
        updateStatus('Ready to train', 'success');

    } catch (error) {
        console.error('Failed to initialize app:', error);
        updateStatus(`Error: ${error.message}`, 'error');
    }
}

// Load and populate examples
async function loadExamples() {
    try {
        const examples = listExamples();
        elements.exampleSelect.innerHTML = '';

        // Cache full example data including inputs and targets for truth table
        window.exampleCache = {};

        examples.forEach(ex => {
            const option = document.createElement('option');
            option.value = ex.name;
            option.textContent = `${ex.name.toUpperCase()} Gate (${ex.architecture.join('-')})`;
            elements.exampleSelect.appendChild(option);

            // Load full example data for truth table display
            try {
                const fullData = getExampleData(ex.name);
                window.exampleCache[ex.name] = fullData;
            } catch (e) {
                console.warn(`Could not load full data for ${ex.name}:`, e);
            }
        });

    } catch (error) {
        console.error('Failed to load examples:', error);
    }
}

// Setup event listeners
function setupEventListeners() {
    elements.exampleSelect.addEventListener('change', updateExampleInfo);
    elements.trainButton.addEventListener('click', startTraining);
    elements.stopButton.addEventListener('click', stopTraining);
    elements.evaluateButton.addEventListener('click', evaluateNetwork);
    elements.input1.addEventListener('input', evaluateNetwork);
    elements.input2.addEventListener('input', evaluateNetwork);

    // Info modal
    document.getElementById('info-button').addEventListener('click', showInfoModal);
    document.getElementById('modal-close').addEventListener('click', hideInfoModal);
    document.getElementById('info-modal').addEventListener('click', (e) => {
        if (e.target.id === 'info-modal') hideInfoModal();
    });
    document.addEventListener('keydown', (e) => {
        if (e.key === 'Escape') hideInfoModal();
    });
}

// Show info modal with educational content
function showInfoModal() {
    const exampleName = elements.exampleSelect.value;
    const education = getExampleEducation(exampleName);

    document.getElementById('modal-title').textContent = education.title;
    document.getElementById('modal-body').innerHTML = education.content;
    document.getElementById('info-modal').classList.add('active');
}

// Hide info modal
function hideInfoModal() {
    document.getElementById('info-modal').classList.remove('active');
}

// Update example information
function updateExampleInfo() {
    try {
        const exampleName = elements.exampleSelect.value;
        const info = getExampleInfo(exampleName);
        currentExampleInfo = info;

        elements.exampleDescription.textContent = `${info.description} | Architecture: [${info.architecture.join(' → ')}]`;

        // Display architecture
        displayArchitecture(info.architecture);

        // Update input/output UI
        updateTestingUI(info.architecture);

    } catch (error) {
        console.error('Failed to update example info:', error);
    }
}

// Display network architecture
function displayArchitecture(layers) {
    const layerNames = ['Input', ...Array(layers.length - 2).fill('Hidden'), 'Output'];

    elements.architectureDisplay.innerHTML = layers.map((size, idx) => `
        <div class="architecture-layer">
            ${layerNames[idx]}<br>
            <small>${size} neuron${size > 1 ? 's' : ''}</small>
        </div>
        ${idx < layers.length - 1 ? '<span class="architecture-arrow">→</span>' : ''}
    `).join('');
}

// Update testing UI based on architecture
function updateTestingUI(architecture) {
    const inputSize = architecture[0];
    const outputSize = architecture[architecture.length - 1];

    const testGrid = document.querySelector('.test-grid');
    testGrid.innerHTML = '';

    // Create input fields
    for (let i = 0; i < inputSize; i++) {
        const inputDiv = document.createElement('div');
        inputDiv.className = 'form-group';
        inputDiv.innerHTML = `
            <label for="input${i}">Input ${i + 1}</label>
            <input type="number" id="input${i}" value="0.0" min="-1" max="1" step="0.1">
        `;
        testGrid.appendChild(inputDiv);
    }

    // Add evaluate button
    const buttonDiv = document.createElement('div');
    buttonDiv.className = 'form-group';
    buttonDiv.innerHTML = `<button id="evaluate-button" class="btn btn-primary" disabled>Evaluate</button>`;
    testGrid.appendChild(buttonDiv);

    // Add output display
    const outputDiv = document.createElement('div');
    outputDiv.className = 'form-group';
    outputDiv.innerHTML = `
        <label>Output</label>
        <div id="output-display" class="output-display">N/A</div>
    `;
    testGrid.appendChild(outputDiv);

    // Re-attach event listeners
    document.getElementById('evaluate-button').addEventListener('click', evaluateNetwork);
    for (let i = 0; i < inputSize; i++) {
        document.getElementById(`input${i}`).addEventListener('input', evaluateNetwork);
    }

    // Update elements reference
    elements.evaluateButton = document.getElementById('evaluate-button');
    elements.outputDisplay = document.getElementById('output-display');
}

// Start training
async function startTraining() {
    const mode = document.querySelector('input[name="mode"]:checked').value;

    if (mode === 'wasm') {
        await startWasmTraining();
    } else {
        await startApiTraining();
    }
}

// Start WASM training (local)
async function startWasmTraining() {
    try {
        const exampleName = elements.exampleSelect.value;
        const epochs = parseInt(elements.epochsInput.value);
        const learningRate = parseFloat(elements.learningRateInput.value);

        // Get optional seed (null if empty or not a valid number)
        // WASM u64 requires BigInt in JavaScript
        const seedValue = elements.seedInput.value.trim();
        const seed = seedValue !== '' ? BigInt(seedValue) : null;

        // Reset state
        lossHistory = [];
        clearChart();

        // Create network with optional seed for reproducibility
        currentNetwork = NeuralNetwork.fromExample(exampleName, learningRate, seed);

        // Update UI
        updateStatus('Training locally with WASM...', 'training');
        elements.trainButton.disabled = true;
        elements.stopButton.disabled = false;
        elements.evaluateButton.disabled = true;
        elements.progressContainer.style.display = 'block';
        trainingStartTime = Date.now();

        // Update timer
        trainingInterval = setInterval(updateTrainingTime, 100);

        // Create progress callback for WASM training
        const progressCallback = (epoch, loss) => {
            updateTrainingProgress(epoch, loss, epochs);
        };

        // Train network (blocking - runs in main thread)
        // Note: In production, this should run in a Web Worker
        await currentNetwork.train(exampleName, epochs, progressCallback);

        // Training complete
        completeTraining();

    } catch (error) {
        console.error('Training failed:', error);
        updateStatus(`Training failed: ${error.message}`, 'error');
        stopTraining();
    }
}

// Start API training (remote with SSE)
async function startApiTraining() {
    try {
        const exampleName = elements.exampleSelect.value;
        const epochs = parseInt(elements.epochsInput.value);
        const learningRate = parseFloat(elements.learningRateInput.value);

        // Get optional seed (null if empty or not a valid number)
        const seedValue = elements.seedInput.value.trim();
        const seed = seedValue !== '' ? parseInt(seedValue) : null;

        // Reset state
        lossHistory = [];
        clearChart();

        // Update UI
        updateStatus('Connecting to training server...', 'training');
        elements.trainButton.disabled = true;
        elements.stopButton.disabled = false;
        elements.evaluateButton.disabled = true;
        elements.progressContainer.style.display = 'block';
        trainingStartTime = Date.now();

        // Update timer
        trainingInterval = setInterval(updateTrainingTime, 100);

        // Build request body with optional seed
        const requestBody = {
            example: exampleName,
            epochs: epochs,
            learning_rate: learningRate
        };
        if (seed !== null) {
            requestBody.seed = seed;
        }

        // Connect to SSE endpoint
        const response = await fetch('/api/train/stream', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify(requestBody)
        });

        if (!response.ok) {
            throw new Error(`Server returned ${response.status}`);
        }

        // Setup SSE reader
        const reader = response.body.getReader();
        const decoder = new TextDecoder();
        let buffer = '';

        updateStatus('Training on server (live updates)...', 'training');

        while (true) {
            const { done, value } = await reader.read();
            if (done) break;

            buffer += decoder.decode(value, { stream: true });
            const lines = buffer.split('\n');
            buffer = lines.pop();

            for (const line of lines) {
                if (line.startsWith('data: ')) {
                    const data = JSON.parse(line.substring(6));
                    updateTrainingProgress(data.epoch, data.loss, epochs);
                }
            }
        }

        // Training complete
        updateStatus('Training completed on server!', 'success');
        completeTraining();

    } catch (error) {
        console.error('API training failed:', error);
        updateStatus(`API training failed: ${error.message}`, 'error');
        stopTraining();
    }
}

// Update training progress
function updateTrainingProgress(epoch, loss, totalEpochs) {
    elements.currentEpoch.textContent = epoch;
    elements.currentLoss.textContent = loss.toFixed(6);

    const progress = (epoch / totalEpochs) * 100;
    elements.progressFill.style.width = `${progress}%`;
    elements.progressText.textContent = `${Math.round(progress)}%`;

    // Add to loss history
    lossHistory.push({ epoch, loss });

    // Update chart every 10 epochs or at the end
    if (epoch % 10 === 0 || epoch === totalEpochs) {
        drawChart();
    }
}

// Complete training
function completeTraining() {
    updateStatus('Training completed!', 'success');
    elements.trainButton.disabled = false;
    elements.stopButton.disabled = true;
    elements.evaluateButton.disabled = false;

    if (trainingInterval) {
        clearInterval(trainingInterval);
        trainingInterval = null;
    }

    // Update truth table
    updateTruthTable();
}

// Stop training
function stopTraining() {
    if (eventSource) {
        eventSource.close();
        eventSource = null;
    }

    if (trainingInterval) {
        clearInterval(trainingInterval);
        trainingInterval = null;
    }

    updateStatus('Training stopped', 'warning');
    elements.trainButton.disabled = false;
    elements.stopButton.disabled = true;
    elements.progressContainer.style.display = 'none';
}

// Update training time
function updateTrainingTime() {
    if (trainingStartTime) {
        const elapsed = (Date.now() - trainingStartTime) / 1000;
        elements.trainingTime.textContent = `${elapsed.toFixed(1)}s`;
    }
}

// Evaluate network
function evaluateNetwork() {
    if (!currentNetwork || !currentExampleInfo) {
        elements.outputDisplay.textContent = 'Train first';
        return;
    }

    try {
        const inputSize = currentExampleInfo.architecture[0];
        const outputSize = currentExampleInfo.architecture[currentExampleInfo.architecture.length - 1];

        // Collect all input values
        const inputs = [];
        for (let i = 0; i < inputSize; i++) {
            const inputElem = document.getElementById(`input${i}`);
            if (inputElem) {
                inputs.push(parseFloat(inputElem.value));
            }
        }

        const outputs = currentNetwork.evaluate(inputs);

        // Display output based on size
        if (outputSize === 1) {
            // Single output - show the value
            elements.outputDisplay.textContent = outputs[0].toFixed(4);
        } else {
            // Multiple outputs - show predicted class (argmax)
            const maxIdx = outputs.indexOf(Math.max(...outputs));
            elements.outputDisplay.textContent = `Class ${maxIdx + 1} (${outputs[maxIdx].toFixed(3)})`;
        }

    } catch (error) {
        console.error('Evaluation failed:', error);
        elements.outputDisplay.textContent = 'Error';
    }
}

// Update truth table
function updateTruthTable() {
    if (!currentNetwork || !currentExampleInfo) return;

    const inputSize = currentExampleInfo.architecture[0];
    const outputSize = currentExampleInfo.architecture[currentExampleInfo.architecture.length - 1];

    // Hide truth table for complex examples (>3 inputs or >4 outputs)
    const truthTableSection = document.getElementById('truth-table');
    if (inputSize > 3 || outputSize > 4) {
        truthTableSection.style.display = 'none';
        return;
    }
    truthTableSection.style.display = 'block';

    // Generate all binary combinations for inputs
    const numCombinations = Math.pow(2, inputSize);
    const testInputs = [];
    for (let i = 0; i < numCombinations; i++) {
        const input = [];
        for (let j = inputSize - 1; j >= 0; j--) {
            input.push((i >> j) & 1 ? 1.0 : 0.0);
        }
        testInputs.push(input);
    }

    // Build table header
    let headerHtml = '<tr>';
    for (let i = 0; i < inputSize; i++) {
        headerHtml += `<th>In ${i + 1}</th>`;
    }
    if (outputSize === 1) {
        headerHtml += '<th>Expected</th><th>Predicted</th><th>Error</th>';
    } else {
        headerHtml += '<th>Expected Class</th><th>Predicted Class</th><th>Confidence</th>';
    }
    headerHtml += '</tr>';

    const thead = elements.truthTableBody.closest('table').querySelector('thead');
    thead.innerHTML = headerHtml;

    // Get example to find expected outputs
    const exampleName = elements.exampleSelect.value;
    const example = window.exampleCache?.[exampleName];

    elements.truthTableBody.innerHTML = testInputs.map((input) => {
        const outputs = currentNetwork.evaluate(input);

        let row = '<tr>';
        // Input columns
        for (let val of input) {
            row += `<td>${val.toFixed(1)}</td>`;
        }

        if (outputSize === 1) {
            // Single output - show value and error
            const predicted = outputs[0];
            // Find expected value if we have training data
            let expected = 0;
            if (example && example.inputs) {
                const matchIdx = example.inputs.findIndex(inp =>
                    inp.every((v, i) => Math.abs(v - input[i]) < 0.01)
                );
                if (matchIdx >= 0 && example.targets[matchIdx]) {
                    expected = example.targets[matchIdx][0];
                }
            }
            const error = Math.abs(predicted - expected);
            const errorClass = error < 0.1 ? 'error-low' : 'error-high';
            row += `<td>${expected.toFixed(1)}</td>`;
            row += `<td>${predicted.toFixed(4)}</td>`;
            row += `<td class="${errorClass}">${error.toFixed(4)}</td>`;
        } else {
            // Multi-output - show predicted class
            const maxIdx = outputs.indexOf(Math.max(...outputs));
            const confidence = outputs[maxIdx];
            // Find expected class
            let expectedClass = 0;
            if (example && example.inputs) {
                const matchIdx = example.inputs.findIndex(inp =>
                    inp.every((v, i) => Math.abs(v - input[i]) < 0.01)
                );
                if (matchIdx >= 0 && example.targets[matchIdx]) {
                    expectedClass = example.targets[matchIdx].indexOf(1.0) + 1;
                }
            }
            const correct = (maxIdx + 1) === expectedClass;
            const classColor = correct ? 'error-low' : 'error-high';
            row += `<td>${expectedClass}</td>`;
            row += `<td class="${classColor}">${maxIdx + 1}</td>`;
            row += `<td>${confidence.toFixed(3)}</td>`;
        }

        row += '</tr>';
        return row;
    }).join('');
}

// Draw loss chart
function drawChart() {
    // Clear canvas
    chartCtx.clearRect(0, 0, chartWidth, chartHeight);

    if (lossHistory.length === 0) return;

    // Calculate scales
    const padding = 40;
    const graphWidth = chartWidth - 2 * padding;
    const graphHeight = chartHeight - 2 * padding;

    const maxLoss = Math.max(...lossHistory.map(h => h.loss));
    const maxEpoch = Math.max(...lossHistory.map(h => h.epoch));

    // Draw axes
    chartCtx.strokeStyle = '#666';
    chartCtx.lineWidth = 2;
    chartCtx.beginPath();
    chartCtx.moveTo(padding, padding);
    chartCtx.lineTo(padding, chartHeight - padding);
    chartCtx.lineTo(chartWidth - padding, chartHeight - padding);
    chartCtx.stroke();

    // Draw grid
    chartCtx.strokeStyle = '#e1e4e8';
    chartCtx.lineWidth = 1;
    for (let i = 0; i <= 5; i++) {
        const y = padding + (graphHeight / 5) * i;
        chartCtx.beginPath();
        chartCtx.moveTo(padding, y);
        chartCtx.lineTo(chartWidth - padding, y);
        chartCtx.stroke();
    }

    // Draw loss line
    chartCtx.strokeStyle = '#4a90e2';
    chartCtx.lineWidth = 2;
    chartCtx.beginPath();

    lossHistory.forEach((point, idx) => {
        const x = padding + (point.epoch / maxEpoch) * graphWidth;
        const y = (chartHeight - padding) - (point.loss / maxLoss) * graphHeight;

        if (idx === 0) {
            chartCtx.moveTo(x, y);
        } else {
            chartCtx.lineTo(x, y);
        }
    });

    chartCtx.stroke();

    // Draw labels
    chartCtx.fillStyle = '#666';
    chartCtx.font = '12px sans-serif';
    chartCtx.textAlign = 'center';

    // X-axis label
    chartCtx.fillText('Epochs', chartWidth / 2, chartHeight - 10);

    // Y-axis label
    chartCtx.save();
    chartCtx.translate(15, chartHeight / 2);
    chartCtx.rotate(-Math.PI / 2);
    chartCtx.fillText('Loss', 0, 0);
    chartCtx.restore();

    // Draw scale values
    chartCtx.textAlign = 'right';
    for (let i = 0; i <= 5; i++) {
        const y = padding + (graphHeight / 5) * i;
        const value = maxLoss * (1 - i / 5);
        chartCtx.fillText(value.toFixed(3), padding - 10, y + 5);
    }

    chartCtx.textAlign = 'center';
    for (let i = 0; i <= 5; i++) {
        const x = padding + (graphWidth / 5) * i;
        const value = Math.round(maxEpoch * (i / 5));
        chartCtx.fillText(value, x, chartHeight - padding + 20);
    }
}

// Clear chart
function clearChart() {
    chartCtx.clearRect(0, 0, chartWidth, chartHeight);

    // Draw placeholder text
    chartCtx.fillStyle = '#999';
    chartCtx.font = '16px sans-serif';
    chartCtx.textAlign = 'center';
    chartCtx.fillText('Training loss will appear here', chartWidth / 2, chartHeight / 2);
}

// Update status message
function updateStatus(message, type = 'info') {
    elements.trainingStatus.textContent = message;

    const statusBox = document.getElementById('training-status');
    statusBox.classList.remove('training-active');

    if (type === 'training') {
        statusBox.classList.add('training-active');
    }
}

// Initialize app when DOM is ready
if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', initApp);
} else {
    initApp();
}
