document.addEventListener('DOMContentLoaded', function() {
    let score = {
        market_cap: null,
        revenue: null,
        profit: null,
        assets: null,
        employees: null
    }

    let timer = 0;
    let timerInterval;
    const timerElement = document.getElementById('timer');

    function startTimer() {
        timerInterval = setInterval(() => {
            timer++;
            timerElement.textContent = `Time spent: ${timer}s`;
        }, 1000);
    }

    startTimer();

    fetch('/company')
        .then(response => response.json())
        .then(data => {
            const companyName = document.getElementById('company-name');
            companyName.textContent = `Guess the financials for: ${data.name}`;

            const companyDescription = document.getElementById('company-description');
            companyDescription.textContent = `Description: ${data.description}\n\n`;

            document.getElementById('market-cap-question').textContent = `Is the market cap lower or higher than ${data.rank <= 250 ? "$40.0B" : "$10.0B"}?`;
            document.getElementById('revenue-question').textContent = `Is the revenue lower or higher than ${data.rank <= 250 ? "$30.0B" : "$7.5B"}?`;
            document.getElementById('profit-question').textContent = `Is the profit lower or higher than ${data.rank <= 250 ? "$10.0B" : "$2.5B"}?`;
            document.getElementById('assets-question').textContent = `Is the value of assets higher or lower than ${data.rank <= 250 ? "$25.0B" : "$6.0B"}?`;
            document.getElementById('employees-question').textContent = `Is the number of employees higher or lower than ${data.rank <= 250 ? "30,000" : "7,500"}?`;
        })
        .catch(error => console.error('Error fetching company:', error));

    // Add event listeners for the buttoms
    document.getElementById('market-cap-higher').addEventListener('click', function() {
        submitGuess('market_cap_higher', 'response-market-cap', ['market-cap-higher', 'market-cap-lower'], 'market_cap');
    });
    document.getElementById('market-cap-lower').addEventListener('click', function() {
        submitGuess('market_cap_lower', 'response-market-cap', ['market-cap-higher', 'market-cap-lower'], 'market_cap');
    });
    document.getElementById('revenue-higher').addEventListener('click', function() {
        submitGuess('revenue_higher', 'response-revenue', ['revenue-higher', 'revenue-lower'], 'revenue');
    });
    document.getElementById('revenue-lower').addEventListener('click', function() {
        submitGuess('revenue_lower', 'response-revenue', ['revenue-higher', 'revenue-lower'], 'revenue');
    });
    document.getElementById('profit-higher').addEventListener('click', function() {
        submitGuess('profit_higher', 'response-profit', ['profit-higher', 'profit-lower'], 'profit');
    });
    document.getElementById('profit-lower').addEventListener('click', function() {
        submitGuess('profit_lower', 'response-profit', ['profit-higher', 'profit-lower'], 'profit');
    });
    document.getElementById('assets-higher').addEventListener('click', function() {
        submitGuess('assets_higher', 'response-assets', ['assets-higher', 'assets-lower'], 'assets');
    });
    document.getElementById('assets-lower').addEventListener('click', function() {
        submitGuess('assets_lower', 'response-assets', ['assets-higher', 'assets-lower'], 'assets');
    });
    document.getElementById('employees-higher').addEventListener('click', function() {
        submitGuess('employees_higher', 'response-employees', ['employees-higher', 'employees-lower'], 'employees');
    });
    document.getElementById('employees-lower').addEventListener('click', function() {
        submitGuess('employees_lower', 'response-employees', ['employees-higher', 'employees-lower'], 'employees');
    });

    document.getElementById('play-again').addEventListener('click', function() {
           location.reload();
       });

    function submitGuess(guess, responseElementId, buttonIdsToDisable, scoreKey){
        fetch('/submit_guess', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/x-www-form-urlencoded',
            },
            body: new URLSearchParams({ guess_type: guess }),
        })
        .then(response => response.text())
        .then(data => {
            const responseElement = document.getElementById(responseElementId);
            responseElement.textContent = data;

            // Remove any existing class
            responseElement.classList.remove('correct', 'incorrect');
            responseElement.classList.add(data.includes('Correct!') ? 'correct' : 'incorrect');
    
            buttonIdsToDisable.forEach(buttonId => {
                document.getElementById(buttonId).disabled = true;
            });
    
            score[scoreKey] = data.includes('Correct!') ? 'Correct!' : 'Incorrect';
    
            if (score.market_cap !== null && score.revenue !== null && score.profit !== null && score.assets !== null && score.employees !== null) {
                displayOverallScore()
                fetch('/stats', {
                    method: 'POST',
                    headers: { 'Content-Type': 'application/json' },
                    body: JSON.stringify({ time: timer })
                })
                .then(response => response.json())
                .then(data => {
                    console.log(data);
                    const statsElement = document.getElementById('overall-stats');
                    let accuracy = (data.correct_guesses / (data.total_games * 5) * 100).toFixed(2);
                    statsElement.textContent = `Overall accuracy: ${accuracy}% Games Played: ${data.total_games}, Correct Guesses: ${data.correct_guesses}, Incorrect Guesses: ${data.incorrect_guesses}`;
                    statsElement.classList.remove('hidden');
                    timerElement.textContent = `Time: ${timer}s. \n Average time per round: ${(data.total_time / data.total_games).toFixed(2)}s`;
                    clearInterval(timerInterval);
                })
                .catch(error => console.error('Error fetching stats:', error));

            }
        })
        .catch(error => {
            console.error('Error', error)
        })
    }
    function displayOverallScore() {
        document.getElementById('response-market-cap').classList.remove('hidden');
        document.getElementById('response-revenue').classList.remove('hidden');
        document.getElementById('response-profit').classList.remove('hidden');
        document.getElementById('response-assets').classList.remove('hidden');
        document.getElementById('response-employees').classList.remove('hidden');

        const overallScore = Object.values(score).filter(value => value === 'Correct!').length;
        const percentage = overallScore / 5 * 100;  
        const overallScoreElement = document.getElementById('overall-score');

        overallScoreElement.textContent = `Your scored: ${percentage}% Share your score on X (formerly Twitter)!`;
        overallScoreElement.classList.remove('hidden');
    }
});




