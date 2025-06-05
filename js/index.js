
// Initial values
const leftPanelContainer = document.getElementById('left-panel');
const toggleState = {};


// Function for loading new page content
const loadPage = async (chapterName) => {
  const filePath = `chapters/${chapterName}.html`;

  fetch(filePath)
    .then(response => {
      if (!response.ok) throw new Error("Page not found");
      return response.text();
    })
    .then(html => {
      document.getElementById('page-mount-point').innerHTML = html;
      Prism.highlightAll();
    })
    .catch(error => {
      console.error(error);
      const mountPoint = document.getElementById('page-mount-point');
      mountPoint.innerHTML = `<p>Error loading ${chapterName}</p>`;
    });
}


const togglePanel = (elemId) => {
  let section = document.getElementById(elemId);
  let currentState = section.style.display;

  // Toggle the selected element
  if (currentState === 'none') {
    currentState = 'block';
    section.style.display = currentState;
  } else {
    currentState = 'none';
    section.style.display = currentState;
  }

  // Toggle off any other element 
  if (!toggleState.hasOwnProperty(elemId)) {
    toggleState[elemId] = currentState; 
  }

  Object.entries(toggleState).forEach(([key, value]) => {
    if (key !== elemId) {
      let closeSection = document.getElementById(key);
      closeSection.style.display = 'none';
      toggleState[elemId] = 'none';
    }
  })
}


const initializeToggleElements = async () => {
  const toggleableElements = document.querySelectorAll('.left-panel-group');
  toggleableElements.forEach(el => {
    toggleState[el.id] = 'none';
    el.style.display = 'none'; 
  }); 
};


document.addEventListener('DOMContentLoaded', async () => {
  await loadPage('intro');
  await initializeToggleElements();
  console.log(toggleState);
});

