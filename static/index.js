document.addEventListener('DOMContentLoaded', function() {
    let score = {
        marketcap: null,
        revenue: null,
        profit: null,
        assets: null,
        employees: null
    };

    let timer = 0;
    let timerInterval;
    const timerElement = document.getElementById('timer');
    let shareText;

    function startTimer() {
        timerInterval = setInterval(() => {
            timer++;
            timerElement.textContent = `Time spent: ${timer}s`;
        }, 1000);
    }

    startTimer();

    fetch('/company', { credentials: 'same-origin' })
        .then(response => response.json())
        .then(data => {
            document.getElementById('company-name').textContent = `Guess the financials for: ${data.name}`;
            document.getElementById('company-description').textContent = `Description: ${data.description}`;
            document.getElementById('marketcap-question').textContent = `Is the market cap lower or higher than ${data.rank <= 250 ? "$40.0B" : "$10.0B"}?`;
            document.getElementById('revenue-question').textContent = `Is the revenue lower or higher than ${data.rank <= 250 ? "$30.0B" : "$7.5B"}?`;
            document.getElementById('profit-question').textContent = `Is the profit lower or higher than ${data.rank <= 250 ? "$10.0B" : "$2.5B"}?`;
            document.getElementById('assets-question').textContent = `Is the value of assets higher or lower than ${data.rank <= 250 ? "$25.0B" : "$6.0B"}?`;
            document.getElementById('employees-question').textContent = `Is the number of employees higher or lower than ${data.rank <= 250 ? "30,000" : "7,500"}?`;
        })
        .catch(error => console.error('Error fetching company:', error));

    function submitGuess(guess, responseElementId, buttonIdsToDisable, scoreKey, selectedButton) {
        fetch('/submit_guess', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/x-www-form-urlencoded',
            },
            credentials: 'same-origin',
            body: new URLSearchParams({ guess_type: guess }),
        })
        .then(response => response.text())
        .then(data => {
            const responseElement = document.getElementById(responseElementId);
            responseElement.textContent = data;
            responseElement.classList.remove('correct', 'incorrect');
            responseElement.classList.add(data.includes('Correct!') ? 'correct' : 'incorrect');

            buttonIdsToDisable.forEach(buttonId => {
                document.getElementById(buttonId).disabled = true;
            });

            selectedButton.classList.add('selected');

            score[scoreKey] = data.includes('Correct!') ? 'Correct!' : 'Incorrect';

            if (score.marketcap !== null && score.revenue !== null && score.profit !== null && score.assets !== null && score.employees !== null) {
                displayOverallScore();
                fetch('/stats', {
                    method: 'POST',
                    headers: { 'Content-Type': 'application/json' },
                    credentials: 'same-origin',
                    body: JSON.stringify({ time: timer })
                })
                .then(response => response.json())
                .then(data => {
                    const statsElement = document.getElementById('overall-stats');
                    let accuracy = (data.correct_guesses / (data.total_games * 5) * 100).toFixed(2);
                    statsElement.textContent = `Overall accuracy: ${accuracy}% Games Played: ${data.total_games}, Correct Guesses: ${data.correct_guesses}, Incorrect Guesses: ${data.incorrect_guesses}`;
                    statsElement.classList.remove('hidden');
                    timerElement.textContent = `Time: ${timer}s. \n Average time per round: ${(data.total_time / data.total_games).toFixed(2)}s`;
                    shareText = getShareText(data);
                    clearInterval(timerInterval);
                })
                .catch(error => console.error('Error fetching stats:', error));
            }
        })
        .catch(error => {
            console.error('Error', error);
        });
    }

    function displayOverallScore() {
        document.getElementById('response-marketcap').classList.remove('hidden');
        document.getElementById('response-revenue').classList.remove('hidden');
        document.getElementById('response-profit').classList.remove('hidden');
        document.getElementById('response-assets').classList.remove('hidden');
        document.getElementById('response-employees').classList.remove('hidden');

        const overallScore = Object.values(score).filter(value => value === 'Correct!').length;
        const percentage = overallScore / 5 * 100;
        const overallScoreElement = document.getElementById('overall-score');

        overallScoreElement.textContent = `Your score: ${percentage}% Share your score on X (formerly Twitter)!`;
        overallScoreElement.classList.remove('hidden');
        document.getElementById('share-results').classList.remove('hidden');
    }

    function getShareText(data) {
        const scoreText = `Bizzle results: ${Object.values(score).filter(value => value === 'Correct!').length}/5`;
        const resultIcons = Object.values(score).map(value => value === 'Correct!' ? '🟩' : '🟥').join('\n');
        let accuracy = (data.correct_guesses / (data.total_games * 5) * 100).toFixed(2);
        let averageTime = (data.total_time / data.total_games).toFixed(2);
        const overallStats = `I've played ${data.total_games} games and with an average of ${averageTime}s per round with ${accuracy}% accuracy`;
        const shareLink = `Can you beat me? \n https://bizzle.onrender.com/`;
        return `${scoreText}\n${resultIcons}\n${overallStats}\n${shareLink}`;
    }

    function copyToClipboard(text) {
        const textarea = document.createElement('textarea');
        textarea.textContent = text;
        document.body.appendChild(textarea);
        textarea.select();
        document.execCommand('copy');
        document.body.removeChild(textarea);
    }

    function isSignal(){
        const userAgent = navigator.userAgent || navigator.vendor || window.opera;
        return userAgent.indexOf('Signal') > -1;
    }

    document.getElementById('share-results').addEventListener('click', function() {
        if (navigator.share) {
            navigator.share({
                // title: 'Bizzle Results',
                text: shareText,
            }).then(() => {
                console.log('Thanks for sharing!');
            }).catch(console.error);
        } else {
            if (isSignal()) {
                const signalShareUrl = `sgnl://send?text=${encodeURIComponent(shareText + ' ' + shareUrl)}`;
                window.location.href = signalShareUrl;
            } else {
            // Fallback for browsers that do not support the Web Share API
            copyToClipboard(shareText);
            alert('Results copied to clipboard!');
            }

        }
    });

    document.querySelectorAll('button[id$="-higher"], button[id$="-lower"]').forEach(button => {
        button.addEventListener('click', function() {
            const [metric, guess] = this.id.split('-');
            const responseElementId = `response-${metric}`;
            const buttonIdsToDisable = [`${metric}-higher`, `${metric}-lower`];
            submitGuess(`${metric}_${guess}`, responseElementId, buttonIdsToDisable, metric, this);
        });
    });

    document.getElementById('play-again').addEventListener('click', function() {
        location.reload();
    });
});
