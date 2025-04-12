using UnityEngine;
using UnityEngine.UI;

public class DiscordAuth : MonoBehaviour
{
    public Button loginButton;
    private string authUrl = "http://localhost:3000/auth/discord";

    void Start()
    {
        if (loginButton != null)
            loginButton.onClick.AddListener(OnLoginClicked);
        else
            Debug.LogError("Login button not assigned!");
    }

    void OnLoginClicked()
    {
        Debug.Log("🌐 Opening Discord login URL...");
        Application.OpenURL(authUrl);
    }
}
