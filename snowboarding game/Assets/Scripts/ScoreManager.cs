using UnityEngine;
using TMPro;
using System;

public class ScoreManager : MonoBehaviour
{
    [Header("UI References")]
    [SerializeField] private TextMeshProUGUI scoreText;
    [SerializeField] private TextMeshProUGUI timerText;

    private int currentScore;
    private float currentTime;
    private bool isTimerRunning = false;

    private void Start()
    {
        currentScore = 0;
        currentTime = 0f;
        UpdateScoreDisplay();
        timerText.gameObject.SetActive(false);
    }

    public void StartTimer()
    {
        isTimerRunning = true;
        timerText.gameObject.SetActive(true);
    }

    private void Update()
    {
        if (isTimerRunning)
        {
            currentTime += Time.deltaTime;
            UpdateTimerDisplay();
        }
    }

    public void AddPoints(int points)
    {
        currentScore += points;
        UpdateScoreDisplay();
    }

    public void StopTimer()
    {
        isTimerRunning = false;
    }

    private void UpdateScoreDisplay()
    {
        scoreText.text = $"Score: {currentScore}";
    }

    private void UpdateTimerDisplay()
    {
        TimeSpan time = TimeSpan.FromSeconds(currentTime);
        timerText.text = $"Time: {time.Minutes:D2}:{time.Seconds:D2}.{(time.Milliseconds / 10):D2}";
    }
}