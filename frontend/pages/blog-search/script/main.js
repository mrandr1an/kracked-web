  document.getElementById("blog-search").addEventListener("click", function(){
       window.location.href = "blogsearch";
   });

  document.getElementById("cv").addEventListener("click", function(){
       window.location.href = "cv";
  })

  document.getElementById("about-blog").addEventListener("click", function(){
       window.location.href = "blogabout";
  })

  document.addEventListener("DOMContentLoaded", function () {
    const searchInput = document.getElementById("search-bar");

    searchInput.addEventListener("input", debounce(handleSearch, 300));

    function handleSearch() {
      const keyword = searchInput.value;
	const backendUrl = 'https://kracked.site/blogsearch';

      fetch(backendUrl, {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
          body: JSON.stringify({"value": keyword}),
      })
        .then(response => response.json())
        .then(data => {
          alert(data.title);
        })
        .catch(error => {
          console.error("Error:", error);
        });
    }

    function debounce(func, delay) {
      let timer;
      return function () {
        const context = this;
        const args = arguments;
        clearTimeout(timer);
        timer = setTimeout(function () {
          func.apply(context, args);
        }, delay);
      };
    }

  });
