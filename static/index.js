document.addEventListener('DOMContentLoaded', function() {
    let score = {
        market_cap: null,
        revenue: null,
        profit: null,
        assets: null,
        employees: null
    }

    fetch('/company')
        .then(response => response.json())
        .then(data => {
            const companyName = document.getElementById('company-name');
            companyName.textContent = `Guess the financials for: ${data.name}`;

            const companyDescription = document.getElementById('company-description');
            companyDescription.textContent = `Description: ${data.description}\n\n`;

            document.getElementById('market-cap-question').textContent = "Is the market cap lower or higher than $40.0B?";
            document.getElementById('revenue-question').textContent = "Is the revenue lower or higher than $30.0B?";
            document.getElementById('profit-question').textContent = "Is the profit lower or higher than $10.0B?";
            document.getElementById('assets-question').textContent = "Is the value of assets higher or lower than $25.0B?";
            document.getElementById('employees-question').textContent = "Is the number of employees higher or lower than 30,000?";
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
            responseElement.classList.remove('hidden');
    
            buttonIdsToDisable.forEach(buttonId => {
                document.getElementById(buttonId).disabled = true;
            });
    
            score[scoreKey] = data.includes('Correct!') ? 'Correct!' : 'Incorrect';
    
            if (score.market_cap !== null && score.revenue !== null && score.profit !== null && score.assets !== null && score.employees !== null) {
                displayOverallScore()
            }
        })
        .catch(error => {
            console.error('Error', error)
        })
    }
    function displayOverallScore() {
        const overallScore = Object.values(score).filter(value => value === 'Correct!').length;
        const percentage = overallScore / 5 * 100;  
        const overallScoreElement = document.getElementById('overall-score');
    
        overallScoreElement.textContent = `Your scored: ${percentage}% Share your score on X (formerly Twitter)!`;
        overallScoreElement.classList.remove('hidden');
    }
});




