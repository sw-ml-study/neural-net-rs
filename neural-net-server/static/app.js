// Neural Network Training Platform - Application Logic
// Minimal JavaScript for bootstrapping WASM and DOM manipulation
// All neural network logic implemented in Rust/WASM

import init, {
    NeuralNetwork,
    listExamples,
    getExampleInfo
} from './wasm/neural_net_wasm.js';

// Application state
let wasmModule;
let currentNetwork = null;
let trainingData = null;
let lossHistory = [];
let trainingStartTime = null;
let trainingInterval = null;
let eventSource = null;

// DOM elements
const elements = {
    exampleSelect: document.getElementById('example-select'),
    exampleDescription: document.getElementById('example-description'),
    epochsInput: document.getElementById('epochs-input'),
    learningRateInput: document.getElementById('learning-rate-input'),
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

        examples.forEach(ex => {
            const option = document.createElement('option');
            option.value = ex.name;
            option.textContent = `${ex.name.toUpperCase()} Gate (${ex.architecture.join('-')})`;
            elements.exampleSelect.appendChild(option);
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
}

// Update example information
function updateExampleInfo() {
    try {
        const exampleName = elements.exampleSelect.value;
        const info = getExampleInfo(exampleName);

        elements.exampleDescription.textContent = `${info.description} | Architecture: [${info.architecture.join(' → ')}]`;

        // Display architecture
        displayArchitecture(info.architecture);

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

        // Reset state
        lossHistory = [];
        clearChart();

        // Create network
        currentNetwork = NeuralNetwork.fromExample(exampleName, learningRate);

        // Update UI
        updateStatus('Training locally with WASM...', 'training');
        elements.trainButton.disabled = true;
        elements.stopButton.disabled = false;
        elements.evaluateButton.disabled = true;
        elements.progressContainer.style.display = 'block';
        trainingStartTime = Date.now();

        // Update timer
        trainingInterval = setInterval(updateTrainingTime, 100);

        // Train network (blocking - runs in main thread)
        // Note: In production, this should run in a Web Worker
        await currentNetwork.train(exampleName, epochs);

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

        // Connect to SSE endpoint
        const response = await fetch('/api/train/stream', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({
                example: exampleName,
                epochs: epochs,
                learning_rate: learningRate
            })
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
    if (!currentNetwork) {
        elements.outputDisplay.textContent = 'Train first';
        return;
    }

    try {
        const input1 = parseFloat(elements.input1.value);
        const input2 = parseFloat(elements.input2.value);

        const output = currentNetwork.evaluate([input1, input2]);
        elements.outputDisplay.textContent = output[0].toFixed(4);

    } catch (error) {
        console.error('Evaluation failed:', error);
        elements.outputDisplay.textContent = 'Error';
    }
}

// Update truth table
function updateTruthTable() {
    if (!currentNetwork) return;

    const inputs = [
        [0.0, 0.0],
        [0.0, 1.0],
        [1.0, 0.0],
        [1.0, 1.0]
    ];

    const exampleName = elements.exampleSelect.value;
    const expectedOutputs = {
        'and': [0.0, 0.0, 0.0, 1.0],
        'or': [0.0, 1.0, 1.0, 1.0],
        'xor': [0.0, 1.0, 1.0, 0.0]
    };

    const expected = expectedOutputs[exampleName] || [0, 0, 0, 0];

    elements.truthTableBody.innerHTML = inputs.map((input, idx) => {
        const output = currentNetwork.evaluate(input);
        const predicted = output[0];
        const error = Math.abs(predicted - expected[idx]);
        const errorClass = error < 0.1 ? 'error-low' : 'error-high';

        return `
            <tr>
                <td>${input[0].toFixed(1)}</td>
                <td>${input[1].toFixed(1)}</td>
                <td>${expected[idx].toFixed(1)}</td>
                <td>${predicted.toFixed(4)}</td>
                <td class="${errorClass}">${error.toFixed(4)}</td>
            </tr>
        `;
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
