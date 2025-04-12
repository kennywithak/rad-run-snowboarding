using TMPro;
using UnityEngine;
using UnityEngine.UI;

public class StartMenuDiscordUI : MonoBehaviour
{
    public GameObject loginButton;             // Reference to the button GameObject
    public TextMeshProUGUI discordNameText;    // Reference to your Text UI

    void Start()
    {
        string username = PlayerPrefs.GetString("DiscordUsername", "");
        string discriminator = PlayerPrefs.GetString("DiscordDiscriminator", "");

        if (!string.IsNullOrEmpty(username))
        {
            // Hide the button
            loginButton.SetActive(false);

            // Show the Discord name text
            discordNameText.gameObject.SetActive(true);
            discordNameText.text = $"Discord: {username}"; // or $"{username}#{discriminator}" if you want full tag
        }
        else
        {
            // Still needs login — show login button and hide name
            loginButton.SetActive(true);
            discordNameText.gameObject.SetActive(false);
        }
    }
}
