      document.body.addEventListener("AppToast", function (evt) {
        console.log("TRIGGERED");
        let msg = evt.detail.message;
        let level = evt.detail.level;
        
        // Create a unique identifier for the toast element
        let toastId = Date.now(); // You can use a different method to generate a unique ID
        
        let toastElementContent = document.createElement("div");
        toastElementContent.classList.add("alert");
        
        // Add the appropriate class based on the toast level
        switch (level) {
          case "Info":
            toastElementContent.classList.add("alert-info");
            break;
          case "Warning":
            toastElementContent.classList.add("alert-warning");
            break;
          case "Error":
            toastElementContent.classList.add("alert-error");
            break;
          case "Success":
            toastElementContent.classList.add("alert-success");
            break;
        }
        
        let toastElementMessage = document.createElement("span");
        toastElementMessage.innerHTML = msg;
        toastElementContent.appendChild(toastElementMessage);
        
        // Append the toast element to the AppToast element
        document.getElementById("AppToast").appendChild(toastElementContent);
        
        setTimeout(function () {
          // Remove the toast element from the AppToast element
          document.getElementById("AppToast").removeChild(toastElementContent);
        }, 2000); // 2000 milliseconds (2 seconds)
      });
