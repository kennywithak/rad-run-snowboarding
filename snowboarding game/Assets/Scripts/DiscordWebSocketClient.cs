using System;
using System.Text;
using UnityEngine;
using System.Net.WebSockets;
using System.Threading;
using System.Threading.Tasks;

public class DiscordWebSocketClient : MonoBehaviour
{
    private ClientWebSocket webSocket;
    private CancellationTokenSource cancellation;
    public static string LoggedInUsername = "";
    public static string Discriminator = "";

    async void Start()
    {
        webSocket = new ClientWebSocket();
        cancellation = new CancellationTokenSource();

        try
        {
            Debug.Log("🧠 Connecting to WebSocket...");
            await webSocket.ConnectAsync(new Uri("ws://localhost:8080"), cancellation.Token);
            Debug.Log("🔌 WebSocket client connected");

            _ = ReceiveLoop(); // fire and forget
        }
        catch (Exception e)
        {
            Debug.LogError("WebSocket connection failed: " + e.Message);
        }
    }

    private async Task ReceiveLoop()
    {
        var buffer = new byte[1024];

        while (webSocket.State == WebSocketState.Open)
        {
            var result = await webSocket.ReceiveAsync(new ArraySegment<byte>(buffer), cancellation.Token);
            if (result.MessageType == WebSocketMessageType.Close)
            {
                Debug.LogWarning("🔌 WebSocket closed by server");
                await webSocket.CloseAsync(WebSocketCloseStatus.NormalClosure, "Closed by client", cancellation.Token);
                return;
            }

            string message = Encoding.UTF8.GetString(buffer, 0, result.Count);
            Debug.Log("📨 WebSocket Message Received: " + message);

            try
            {
                DiscordUser user = JsonUtility.FromJson<DiscordUser>(message);

                // Use Dispatcher to safely access Unity API
                MainThreadDispatcher.Enqueue(() =>
                {
                    LoggedInUsername = user.username;
                    Discriminator = user.discriminator;
                    Debug.Log($"✅ Logged in as {LoggedInUsername}#{Discriminator}");
                });
            }
            catch (Exception e)
            {
                Debug.LogError("❌ Failed to parse WebSocket message: " + e.Message);
            }
        }
    }

    private void OnDestroy()
    {
        cancellation?.Cancel();
        webSocket?.Dispose();
    }
}
