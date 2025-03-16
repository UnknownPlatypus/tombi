const clipIcon = `<svg fill="currentColor" stroke-width="0" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 16 16" height="1em" width="1em" style="overflow: visible; color: currentcolor;"><path d="m7.775 3.275 1.25-1.25a3.5 3.5 0 1 1 4.95 4.95l-2.5 2.5a3.5 3.5 0 0 1-4.95 0 .751.751 0 0 1 .018-1.042.751.751 0 0 1 1.042-.018 1.998 1.998 0 0 0 2.83 0l2.5-2.5a2.002 2.002 0 0 0-2.83-2.83l-1.25 1.25a.751.751 0 0 1-1.042-.018.751.751 0 0 1-.018-1.042Zm-4.69 9.64a1.998 1.998 0 0 0 2.83 0l1.25-1.25a.751.751 0 0 1 1.042.018.751.751 0 0 1 .018 1.042l-1.25 1.25a3.5 3.5 0 1 1-4.95-4.95l2.5-2.5a3.5 3.5 0 0 1 4.95 0 .751.751 0 0 1-.018 1.042.751.751 0 0 1-1.042.018 1.998 1.998 0 0 0-2.83 0l-2.5 2.5a1.998 1.998 0 0 0 0 2.83Z"></path></svg>`;

const checkIcon = `<svg fill="none" stroke-width="2" xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-check" width="1em" height="1em" viewBox="0 0 24 24" stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" style="overflow: visible; color: #22c55e;"><path stroke="none" d="M0 0h24v24H0z" fill="none"></path><path d="M5 12l5 5l10 -10"></path></svg>`;

export function setupAnchors() {
  for (const heading of document.querySelectorAll(".heading-with-anchor")) {
    const anchorLink = document.createElement("a");
    anchorLink.className = "anchor-link";
    anchorLink.setAttribute("data-anchor", "true");
    anchorLink.setAttribute("aria-hidden", "true");
    anchorLink.innerHTML = clipIcon;
    heading.appendChild(anchorLink);
  }

  document.addEventListener("click", (event) => {
    const target = event.target as HTMLElement;
    const anchorLink = target.closest(".anchor-link");
    if (anchorLink) {
      const heading = target.closest(".heading-with-anchor") as HTMLElement;
      if (heading?.id) {
        event.preventDefault();
        const url = `${window.location.href.split("#")[0]}#${heading.id}`;
        navigator.clipboard.writeText(url).then(() => {
          anchorLink.innerHTML = checkIcon;

          setTimeout(() => {
            anchorLink.innerHTML = clipIcon;
          }, 500);
        });
      }
    }
  });
}
