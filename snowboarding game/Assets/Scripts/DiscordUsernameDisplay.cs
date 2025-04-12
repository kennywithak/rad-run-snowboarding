using TMPro;
using UnityEngine;

public class DiscordUsernameDisplay : MonoBehaviour
{
    public TextMeshProUGUI usernameText;

    void Start()
    {
        string username = PlayerPrefs.GetString("DiscordUsername", "Guest");
        string discriminator = PlayerPrefs.GetString("DiscordDiscriminator", "0000");

        usernameText.text = $"Discord: {username}";
    }
}
