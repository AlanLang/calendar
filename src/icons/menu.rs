use leptos::*;

#[component]
pub fn MenuIcon() -> impl IntoView {
  view! {
    <button
      aria-label="Hide Calendar menu"
      class="group h-[20px] w-[20px] text-sm p-0.5 text-object-default hover:text-gray-500"
      type="button"
    >
      <div class="sc-ku1yu7-2 dpUhKv">
        <svg
          viewBox="0 0 16 16"
          fill="none"
          xmlns="http://www.w3.org/2000/svg"
          class="sc-seair9-0 hcnghW"
        >
          <path
            class="Sidebar group-hover:fill-current"
            clip-rule="evenodd"
            d="M2.5 2H7v12H2.5A1.5 1.5 0 0 1 1 12.5v-9A1.5 1.5 0 0 1 2.5 2ZM3 4.5A.25.25 0 0 0 3 5h2a.25.25 0 0 0 0-.5H3Zm0 2A.25.25 0 0 0 3 7h2a.25.25 0 0 0 0-.5H3Zm0 2A.25.25 0 0 0 3 9h2a.25.25 0 0 0 0-.5H3Z"
            fill="none"
          ></path>
          <path
            class="Menu group-hover:opacity-0"
            d="M3 4.75h2m-2 2h2m-2 2h2"
            stroke="currentColor"
            stroke-width="0.5"
            stroke-linecap="round"
          ></path>
          <path
            d="M2.5 2.5h11a1 1 0 0 1 1 1v9a1 1 0 0 1-1 1h-11a1 1 0 0 1-1-1v-9a1 1 0 0 1 1-1ZM6.5 2v12"
            stroke="currentColor"
          ></path>
        </svg>
      </div>
    </button>
  }
}
