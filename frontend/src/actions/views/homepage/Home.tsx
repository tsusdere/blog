import BasicBreadcrumbs from '../../components/Breadcrumbs.tsx'
const Home = () => {
  return (
    <div class="flex flex-col space-y-135">
      <div class="bg-blue-500 relative top-40">
        <BasicBreadcrumbs/>
      </div>
      <div class="bg-black relative bottom-30">02</div>
      <div class="bg-black relative bottom-30">03</div>
    </div>
  )
};

export default Home;
