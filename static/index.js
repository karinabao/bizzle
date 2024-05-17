document.addEventListener('DOMContentLoaded', function() {
    fetchCompanyInfo();

    // Auto-focus on the first input box when the page loads
    const firstGuessInput = document.getElementById('first-guess');
    firstGuessInput.focus();
});

document.getElementById('guessForm').addEventListener('submit', function(event) {
    event.preventDefault();
    handleSubmitGuess();
});

function fetchCompanyInfo() {
    fetch('/company')
        .then(response => response.json())
        .then(data => {
            const companyInfo = document.getElementById('company-info');
            companyInfo.textContent = `Guess the market cap for: ${data.name}`;
        })
        .catch(error => console.error('Error fetching company:', error));
}

function handleSubmitGuess() {
    const guessInputs = [
        document.getElementById('first-guess'),
        document.getElementById('second-guess'),
        document.getElementById('third-guess'),
        document.getElementById('fourth-guess')
    ];
    const responseIds = [
        'first-response',
        'second-response',
        'third-response',
        'fourth-response'
    ];

    let guess, guessNumber;

    for (let i = 0; i < guessInputs.length; i++) {
        if (!guessInputs[i].disabled) {
            guess = guessInputs[i].value;
            guessNumber = i + 1;
            break;
        }
    }

    fetch('/submit_guess', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/x-www-form-urlencoded',
        },
        body: new URLSearchParams({ guess, guessNumber }),
    })
    .then(response => response.text())
    .then(data => {
        updateResponseMessage(responseIds[guessNumber - 1], data);
        if (data.includes('Congratulations!')) {
            disableAllInputs(guessInputs);
            document.getElementById('submit-btn').disabled = true;
        } else if (guessNumber < guessInputs.length) {
            setNextGuessInput(guessInputs, guessNumber);
        } else {
            disableAllInputs(guessInputs);
            document.getElementById('submit-btn').disabled = true;
        }
    })
    .catch(error => {
        console.error('Error:', error);
    });
}

function setNextGuessInput(guessInputs, guessNumber) {
    guessInputs[guessNumber - 1].disabled = true;
    guessInputs[guessNumber].disabled = false;
    guessInputs[guessNumber].focus();
}

function updateResponseMessage(responseId, message) {
    document.getElementById(responseId).textContent = message;
}

function disableAllInputs(guessInputs) {
    for (const input of guessInputs) {
        input.disabled = true;
    }
}
